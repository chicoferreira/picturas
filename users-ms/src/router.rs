use crate::error::{AppError, AppResult};
use crate::{jwt, password, redis, user, AppState};
use axum::extract::State;
use axum::http::{header, HeaderMap};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{debug_handler, Json, Router};
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::CookieJar;
use chrono::{DateTime, Utc};
use tower_http::trace::TraceLayer;
use uuid::Uuid;
use validator::Validate;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/api/v1/users/register", post(register_user))
        .route("/api/v1/users/login", post(login_user))
        .route("/api/v1/users/refresh", post(refresh_access_token))
        .route("/api/v1/users/me", get(get_current_user))
        .route("/api/v1/users/logout", post(logout_user))
        .route("/api/v1/users/changepassword", post(change_password))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
}

const ACCESS_TOKEN_COOKIE_NAME: &str = "access_token";
const REFRESH_TOKEN_COOKIE_NAME: &str = "refresh_token";

#[derive(serde::Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 50))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, max = 50))]
    pub password: String,
}

#[derive(serde::Serialize)]
struct RegisterResponse {
    uuid: Uuid,
    name: String,
    email: String,
    created_at: DateTime<Utc>,
    access_token: String,
    refresh_token: String,
}

#[debug_handler]
async fn register_user(
    State(state): State<AppState>,
    Json(request): Json<RegisterRequest>,
) -> AppResult<impl IntoResponse> {
    request.validate()?;

    if user::get_user_from_email(request.email.clone(), &state)
        .await?
        .is_some()
    {
        // email already exists
        return Err(AppError::EmailAlreadyInUse(request.email));
    }

    let user = user::register_user(request, &state).await?;

    let token_id = Uuid::new_v4();
    let refresh_token = jwt::create_refresh_token(&state, token_id, &user)?;
    let access_token = jwt::create_access_token(&state, token_id, &user)?;

    redis::store_token(token_id, user.uuid, &state).await?;

    let mut headers = HeaderMap::new();
    append_access_token_cookie(&mut headers, &access_token, &state);
    append_refresh_token_cookie(&mut headers, &refresh_token, &state);

    Ok((
        headers,
        Json(RegisterResponse {
            uuid: user.uuid,
            name: user.name,
            email: user.email,
            created_at: user.created_at,
            access_token,
            refresh_token,
        }),
    ))
}

#[derive(serde::Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(serde::Serialize)]
struct LoginResponse {
    access_token: String,
    refresh_token: String,
}

#[debug_handler]
async fn login_user(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> AppResult<impl IntoResponse> {
    let user = user::get_user_from_email(request.email.clone(), &state)
        .await?
        .ok_or(AppError::EmailNotFound(request.email))?;

    if !password::verify_password(&request.password, &user.password)? {
        return Err(AppError::InvalidPassword);
    }

    let token_id = Uuid::new_v4();
    let access_token = jwt::create_access_token(&state, token_id, &user)?;
    let refresh_token = jwt::create_refresh_token(&state, token_id, &user)?;

    redis::store_token(token_id, user.uuid, &state).await?;

    let mut headers = HeaderMap::new();
    append_access_token_cookie(&mut headers, &access_token, &state);
    append_refresh_token_cookie(&mut headers, &refresh_token, &state);

    Ok((
        headers,
        Json(LoginResponse {
            access_token,
            refresh_token,
        }),
    ))
}

#[derive(serde::Serialize)]
struct RefreshAccessTokenResponse {
    access_token: String,
}

#[debug_handler]
async fn refresh_access_token(
    cookie_jar: CookieJar,
    State(state): State<AppState>,
) -> AppResult<impl IntoResponse> {
    let refresh_token = cookie_jar
        .get(REFRESH_TOKEN_COOKIE_NAME)
        .ok_or(AppError::InvalidToken)?;

    let token = jwt::decode_refresh_token(&state, refresh_token.value())?;

    let user = user::get_user_by_uuid(token.sub, &state)
        .await?
        .ok_or(AppError::InvalidToken)?;

    let user_uuid_from_redis = redis::get_user_id_from_token(token.token_id, &state)
        .await?
        .ok_or(AppError::InvalidToken)?;

    if user.uuid != user_uuid_from_redis {
        return Err(AppError::InvalidToken);
    }

    let access_token = jwt::create_access_token(&state, token.token_id, &user)?;

    let mut headers = HeaderMap::new();
    append_access_token_cookie(&mut headers, &access_token, &state);

    Ok((headers, Json(RefreshAccessTokenResponse { access_token })))
}

#[derive(serde::Serialize)]
struct CurrentUserResponse {
    uuid: Uuid,
    name: String,
    email: String,
    created_at: DateTime<Utc>,
}

#[debug_handler]
async fn get_current_user(
    cookie_jar: CookieJar,
    State(state): State<AppState>,
) -> AppResult<Json<CurrentUserResponse>> {
    let access_token = cookie_jar
        .get(ACCESS_TOKEN_COOKIE_NAME)
        .ok_or(AppError::Unauthorized)?;

    let token = jwt::decode_access_token(&state, access_token.value())?;

    let user = user::get_user_by_uuid(token.sub, &state)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let user = CurrentUserResponse {
        uuid: user.uuid,
        name: user.name,
        email: user.email,
        created_at: user.created_at,
    };

    Ok(Json(user))
}

#[debug_handler]
async fn logout_user(
    cookie_jar: CookieJar,
    State(state): State<AppState>,
) -> AppResult<impl IntoResponse> {
    let refresh_token = cookie_jar
        .get(REFRESH_TOKEN_COOKIE_NAME)
        .ok_or(AppError::InvalidToken)?;

    let token = jwt::decode_refresh_token(&state, refresh_token.value())?;
    let user = user::get_user_by_uuid(token.sub, &state)
        .await?
        .ok_or(AppError::Unauthorized)?;

    redis::delete_token(token.token_id, &state).await?;

    let mut headers = HeaderMap::new();

    let mut append_empty_cookie = |name: &str| {
        headers.append(
            header::SET_COOKIE,
            Cookie::build((name, ""))
                .http_only(true)
                .max_age(time::Duration::seconds(-1))
                .path("/")
                .build()
                .to_string()
                .parse()
                .unwrap(),
        );
    };

    append_empty_cookie(ACCESS_TOKEN_COOKIE_NAME);
    append_empty_cookie(REFRESH_TOKEN_COOKIE_NAME);

    let user = CurrentUserResponse {
        uuid: user.uuid,
        name: user.name,
        email: user.email,
        created_at: user.created_at,
    };

    Ok((headers, Json(user)))
}

#[derive(serde::Deserialize, Validate)]
struct ChangePasswordRequest {
    #[validate(length(min = 8, max = 50))]
    current_password: String,
    #[validate(length(min = 8, max = 50))]
    new_password: String,
}

#[derive(serde::Serialize)]
struct ChangePasswordResponse {
    access_token: String,
    refresh_token: String,
}

#[debug_handler]
async fn change_password(
    cookie_jar: CookieJar,
    State(state): State<AppState>,
    Json(request): Json<ChangePasswordRequest>,
) -> AppResult<impl IntoResponse> {
    let access_token = cookie_jar
        .get(ACCESS_TOKEN_COOKIE_NAME)
        .ok_or(AppError::InvalidToken)?;

    let token = jwt::decode_access_token(&state, access_token.value())?;
    let user = user::get_user_by_uuid(token.sub, &state)
        .await?
        .ok_or(AppError::Unauthorized)?;

    if !password::verify_password(&request.current_password, &user.password)? {
        return Err(AppError::InvalidPassword);
    }

    redis::delete_token(token.token_id, &state).await?;
    user::change_password(user.uuid, request.new_password.clone(), &state).await?;
    let token_id = Uuid::new_v4();
    let access_token = jwt::create_access_token(&state, token_id, &user)?;
    let refresh_token = jwt::create_refresh_token(&state, token_id, &user)?;

    redis::store_token(token_id, user.uuid, &state).await?;

    let mut headers = HeaderMap::new();
    append_access_token_cookie(&mut headers, &access_token, &state);
    append_refresh_token_cookie(&mut headers, &refresh_token, &state);

    Ok((
        headers,
        Json(ChangePasswordResponse {
            access_token,
            refresh_token,
        }),
    ))
}

fn append_access_token_cookie(
    header_map: &mut HeaderMap,
    access_token: &str,
    app_state: &AppState,
) {
    let access_cookie = Cookie::build((ACCESS_TOKEN_COOKIE_NAME, access_token.to_string()))
        .http_only(true)
        .max_age(time::Duration::try_from(app_state.config.access_token_max_age).unwrap())
        .path("/");
    header_map.append(
        header::SET_COOKIE,
        access_cookie.to_string().parse().unwrap(),
    );
}

fn append_refresh_token_cookie(
    header_map: &mut HeaderMap,
    refresh_token: &str,
    app_state: &AppState,
) {
    let refresh_cookie = Cookie::build((REFRESH_TOKEN_COOKIE_NAME, refresh_token.to_string()))
        .http_only(true)
        .max_age(time::Duration::try_from(app_state.config.refresh_token_max_age).unwrap())
        .path("/");
    header_map.append(
        header::SET_COOKIE,
        refresh_cookie.to_string().parse().unwrap(),
    );
}
