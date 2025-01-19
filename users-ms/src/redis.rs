use crate::error::{AppError, AppResult};
use crate::AppState;
use rustis::commands::{GenericCommands, StringCommands};
use uuid::Uuid;

pub async fn store_token(token_id: Uuid, user_id: Uuid, state: &AppState) -> AppResult<()> {
    state
        .redis_client
        .setex(
            token_id.to_string(),
            state.config.refresh_token_max_age.as_secs(),
            user_id.to_string(),
        )
        .await
        .map_err(AppError::RedisError)
}

pub async fn get_user_id_from_token(token_id: Uuid, state: &AppState) -> AppResult<Option<Uuid>> {
    let user_id: String = state
        .redis_client
        .get(token_id.to_string())
        .await
        .map_err(AppError::RedisError)?;

    dbg!(&user_id);

    Ok(Uuid::parse_str(&user_id).ok())
}

pub async fn delete_token(token_id: Uuid, state: &AppState) -> AppResult<usize> {
    state
        .redis_client
        .del(token_id.to_string())
        .await
        .map_err(AppError::RedisError)
}
