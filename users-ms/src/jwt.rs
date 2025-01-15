use crate::error::AppResult;
use crate::user::User;
use crate::AppState;
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header};
use serde::Serialize;
use std::ops::Add;
use uuid::Uuid;

#[derive(Serialize)]
struct AccessTokenClaims {
    sub: Uuid,
    name: String,
    email: String,
    exp: i64,
}

#[derive(Serialize)]
struct RefreshTokenClaims {
    sub: Uuid,
    token_id: Uuid,
    exp: i64,
}

pub fn create_access_token(state: &AppState, user: &User) -> AppResult<String> {
    let expiration = Utc::now().add(Duration::minutes(15)).timestamp();

    let claims = AccessTokenClaims {
        sub: user.uuid,
        name: user.name.clone(),
        email: user.email.clone(),
        exp: expiration,
    };

    let token = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.jwt_secret.as_ref()),
    )?;
    Ok(token)
}

pub fn create_refresh_token(state: &AppState, user: &User) -> AppResult<String> {
    let expiration = Utc::now().add(Duration::days(7)).timestamp();

    let claims = RefreshTokenClaims {
        sub: user.uuid,
        token_id: Uuid::new_v4(),
        exp: expiration,
    };

    let token = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.jwt_secret.as_ref()),
    )?;
    Ok(token)
}

