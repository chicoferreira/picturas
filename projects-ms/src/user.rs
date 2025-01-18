use crate::error::AppError;
use crate::error::Result;
use crate::state::AppState;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum_extra::extract::CookieJar;
use chrono::Utc;
use jsonwebtoken::{Algorithm, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

impl FromRequestParts<AppState> for AccessTokenClaims {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self> {
        CookieJar::from_headers(&parts.headers)
            .get("access_token")
            .ok_or(AppError::Unauthorized)
            .and_then(|cookie| {
                let token = cookie.value();
                let token = decode_access_token(state, token)?;
                Ok(token)
            })
    }
}

#[derive(Serialize, Deserialize)]
pub struct AccessTokenClaims {
    pub sub: Uuid,
    pub name: String,
    pub email: String,
    pub exp: i64,
}

pub fn decode_access_token(state: &AppState, token: &str) -> Result<AccessTokenClaims> {
    let key = &state.config.access_token_public_key;
    let token: AccessTokenClaims =
        jsonwebtoken::decode(token, key, &Validation::new(Algorithm::RS256))?.claims;
    validate_expiration_date(token.exp)?;
    Ok(token)
}

fn validate_expiration_date(expiration: i64) -> Result<()> {
    let now = Utc::now().timestamp();
    if now > expiration {
        return Err(AppError::Unauthorized);
    }
    Ok(())
}
