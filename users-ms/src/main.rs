mod config;
mod error;
mod jwt;
mod password;
mod user;

use crate::error::{AppError, AppResult};
use axum::extract::State;
use axum::routing::post;
use axum::{debug_handler, Json, Router};
use chrono::{DateTime, Utc};
use clap::Parser;
use sqlx::PgPool;
use tower_http::trace::TraceLayer;
use uuid::Uuid;
use validator::Validate;

#[derive(Clone)]
struct AppState {
    pg_pool: PgPool,
    jwt_secret: Vec<u8>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = config::Config::parse();

    let listener = tokio::net::TcpListener::bind(config.bind_addr)
        .await
        .expect("Failed to bind to port");

    let options = sqlx::postgres::PgConnectOptions::new()
        .host(&config.pg_host)
        .port(config.pg_port)
        .username(&config.pg_user)
        .password(&config.pg_password)
        .database("picturas");

    let pool = PgPool::connect_with(options)
        .await
        .expect("Failed to connect to Postgres");

    let state = AppState {
        pg_pool: pool,
        jwt_secret: config.jwt_secret,
    };

    let router = Router::new()
        .route("/users/register", post(register_user))
        .route("/users/login", post(login_user))
        // TODO: missing route for refreshing tokens and validating access tokens
        // TODO: missing route for getting current user info
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    axum::serve(listener, router).await.unwrap();
}

#[derive(serde::Deserialize, Validate)]
struct RegisterRequest {
    #[validate(length(min = 3, max = 50))]
    name: String,
    #[validate(email)]
    email: String,
    #[validate(length(min = 8, max = 50))]
    password: String,
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
) -> AppResult<Json<RegisterResponse>> {
    request.validate()?;

    if user::get_user_from_email(request.email.clone(), &state)
        .await?
        .is_some()
    {
        // email already exists
        return Err(AppError::EmailAlreadyInUse(request.email));
    }

    let user = user::register_user(request, &state).await?;

    let access_token = jwt::create_access_token(&state, &user)?;
    let refresh_token = jwt::create_refresh_token(&state, &user)?;

    Ok(Json(RegisterResponse {
        uuid: user.uuid,
        name: user.name,
        email: user.email,
        created_at: user.created_at,
        access_token,
        refresh_token,
    }))
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
) -> AppResult<Json<LoginResponse>> {
    let user = user::get_user_from_email(request.email.clone(), &state)
        .await?
        .ok_or(AppError::EmailNotFound(request.email))?;

    if !password::verify_password(&request.password, &user.password)? {
        return Err(AppError::InvalidPassword);
    }

    let access_token = jwt::create_access_token(&state, &user)?;
    let refresh_token = jwt::create_refresh_token(&state, &user)?;

    Ok(Json(LoginResponse {
        access_token,
        refresh_token,
    }))

    // TODO: set cookies for access and refresh tokens
}
