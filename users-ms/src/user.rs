use crate::{password, AppResult, AppState, RegisterRequest};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub enum UserRole {
    Default,
    Premium,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub uuid: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
}

pub async fn get_user_from_email(email: String, state: &AppState) -> AppResult<Option<User>> {
    Ok(
        sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", email)
            .fetch_optional(&state.pg_pool)
            .await?,
    )
}

pub async fn register_user(user: RegisterRequest, state: &AppState) -> AppResult<User> {
    let password = password::hash_password(&user.password)?;

    let user = User {
        uuid: Uuid::new_v4(),
        name: user.name,
        email: user.email,
        password,
        created_at: Utc::now(),
    };

    sqlx::query!(
        "INSERT INTO users (uuid, name, email, password, created_at) VALUES ($1, $2, $3, $4, $5)",
        user.uuid,
        user.name,
        user.email,
        user.password,
        user.created_at
    )
    .execute(&state.pg_pool)
    .await?;

    Ok(user)
}
