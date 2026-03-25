use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id:            Uuid,
    pub email:         String,
    pub username:      String,
    pub display_name:  String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub dark_matter:   i64,
    pub is_banned:     bool,
    pub ban_reason:    Option<String>,
    pub is_admin:      bool,
    pub last_login_at: Option<DateTime<Utc>>,
    pub created_at:    DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct RegisterDto {
    pub email:        String,
    pub username:     String,
    pub display_name: String,
    pub password:     String,
}

#[derive(Debug, Deserialize)]
pub struct LoginDto {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserPublic {
    pub id:           Uuid,
    pub username:     String,
    pub display_name: String,
    pub dark_matter:  i64,
    pub created_at:   DateTime<Utc>,
}

impl From<User> for UserPublic {
    fn from(u: User) -> Self {
        UserPublic {
            id:           u.id,
            username:     u.username,
            display_name: u.display_name,
            dark_matter:  u.dark_matter,
            created_at:   u.created_at,
        }
    }
}
