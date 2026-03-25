use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::user::{LoginDto, RegisterDto, User};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub:      Uuid,
    pub is_admin: bool,
    pub exp:      i64,
    pub iat:      i64,
}

impl Claims {
    pub fn new(user_id: Uuid, is_admin: bool, ttl_hours: i64) -> Self {
        let now = Utc::now();
        Self {
            sub:      user_id,
            is_admin,
            exp:      (now + Duration::hours(ttl_hours)).timestamp(),
            iat:      now.timestamp(),
        }
    }

    pub fn encode(&self, secret: &str) -> Result<String, AppError> {
        encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .map_err(|e| AppError::Internal(e.to_string()))
    }

    pub fn decode(token: &str, secret: &str) -> Result<Self, jsonwebtoken::errors::Error> {
        let data = decode::<Self>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )?;
        Ok(data.claims)
    }
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token:        String,
    pub refresh_token: String,
    pub user_id:      Uuid,
    pub username:     String,
    pub display_name: String,
    pub is_admin:     bool,
}

pub async fn register(pool: &PgPool, dto: RegisterDto, jwt_secret: &str) -> Result<AuthResponse, AppError> {
    // Check uniqueness
    let exists: bool = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM users WHERE email = $1 OR username = $2)",
        dto.email,
        dto.username,
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::from)?
    .unwrap_or(false);

    if exists {
        return Err(AppError::Conflict("Email or username already taken".into()));
    }

    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(dto.password.as_bytes(), &salt)
        .map_err(|e| AppError::Internal(e.to_string()))?
        .to_string();

    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (email, username, display_name, password_hash)
        VALUES ($1, $2, $3, $4)
        RETURNING
            id, email, username, display_name, password_hash,
            dark_matter, is_banned, ban_reason, is_admin,
            last_login_at, created_at
        "#,
        dto.email,
        dto.username,
        dto.display_name,
        hash,
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::from)?;

    issue_tokens(&user, jwt_secret)
}

pub async fn login(pool: &PgPool, dto: LoginDto, jwt_secret: &str) -> Result<AuthResponse, AppError> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT
            id, email, username, display_name, password_hash,
            dark_matter, is_banned, ban_reason, is_admin,
            last_login_at, created_at
        FROM users WHERE username = $1
        "#,
        dto.username,
    )
    .fetch_optional(pool)
    .await
    .map_err(AppError::from)?
    .ok_or_else(|| AppError::Unauthorized("Invalid credentials".into()))?;

    if user.is_banned {
        return Err(AppError::Forbidden(format!(
            "Account banned: {}",
            user.ban_reason.unwrap_or_default()
        )));
    }

    let parsed_hash = PasswordHash::new(&user.password_hash)
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Argon2::default()
        .verify_password(dto.password.as_bytes(), &parsed_hash)
        .map_err(|_| AppError::Unauthorized("Invalid credentials".into()))?;

    // Update last_login_at
    sqlx::query!("UPDATE users SET last_login_at = NOW() WHERE id = $1", user.id)
        .execute(pool)
        .await
        .map_err(AppError::from)?;

    issue_tokens(&user, jwt_secret)
}

fn issue_tokens(user: &User, jwt_secret: &str) -> Result<AuthResponse, AppError> {
    let token = Claims::new(user.id, user.is_admin, 1).encode(jwt_secret)?;
    let refresh_token = Claims::new(user.id, user.is_admin, 30 * 24).encode(jwt_secret)?;

    Ok(AuthResponse {
        token,
        refresh_token,
        user_id:      user.id,
        username:     user.username.clone(),
        display_name: user.display_name.clone(),
        is_admin:     user.is_admin,
    })
}

pub async fn refresh(pool: &PgPool, refresh_token: &str, jwt_secret: &str) -> Result<AuthResponse, AppError> {
    let claims = Claims::decode(refresh_token, jwt_secret)
        .map_err(|_| AppError::Unauthorized("Invalid refresh token".into()))?;

    let user = sqlx::query_as!(
        User,
        r#"
        SELECT
            id, email, username, display_name, password_hash,
            dark_matter, is_banned, ban_reason, is_admin,
            last_login_at, created_at
        FROM users WHERE id = $1
        "#,
        claims.sub,
    )
    .fetch_optional(pool)
    .await
    .map_err(AppError::from)?
    .ok_or_else(|| AppError::Unauthorized("User not found".into()))?;

    if user.is_banned {
        return Err(AppError::Forbidden("Account banned".into()));
    }

    issue_tokens(&user, jwt_secret)
}
