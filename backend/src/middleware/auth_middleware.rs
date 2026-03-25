use axum::{
    extract::{Extension, Request},
    http::header,
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

use crate::error::AppError;
use crate::services::auth_service::Claims;

/// Carries authenticated user info — inserted into request extensions by `auth_layer`.
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id:  Uuid,
    pub is_admin: bool,
}

/// Convenient type alias for handler extraction.
pub type ExtractAuth = Extension<AuthUser>;

/// Middleware: validates Bearer JWT and inserts `AuthUser` into request extensions.
pub async fn auth_layer(mut req: Request, next: Next) -> Result<Response, AppError> {
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or_else(|| AppError::Unauthorized("Missing Bearer token".into()))?;

    let secret = std::env::var("JWT_SECRET").unwrap_or_default();
    let claims = Claims::decode(token, &secret)
        .map_err(|_| AppError::Unauthorized("Invalid or expired token".into()))?;

    req.extensions_mut().insert(AuthUser {
        user_id:  claims.sub,
        is_admin: claims.is_admin,
    });

    Ok(next.run(req).await)
}
