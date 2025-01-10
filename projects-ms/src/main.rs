mod error;
mod image;
mod model;
mod project;
mod router;
mod user;

use axum::extract::Path;
use axum::http::StatusCode;
use clap::Parser;
use sqlx::postgres::PgConnectOptions;
use sqlx::PgPool;
use std::path::PathBuf;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
struct AppState {
    db_pool: PgPool,
    config: Arc<Config>,
}

#[derive(Debug, Parser)]
struct Config {
    #[arg(long, env)]
    pg_host: String,
    #[arg(long, env, default_value_t = 5432)]
    pg_port: u16,
    #[arg(long, env)]
    pg_user: String,
    #[arg(long, env)]
    pg_password: String,
    #[arg(long, env)]
    picturas_bind_address: String,
    #[arg(long, env)]
    picturas_image_folder: PathBuf,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let config = Config::parse();

    let conn = PgConnectOptions::new()
        .host(&config.pg_host)
        .port(config.pg_port)
        .username(&config.pg_user)
        .password(&config.pg_password)
        .database("picturas");

    let pg_pool = PgPool::connect_with(conn)
        .await
        .expect("Failed to connect to Postgres.");

    let bind_address = &config.picturas_bind_address;
    let listener = tokio::net::TcpListener::bind(bind_address).await.unwrap();

    let state = AppState {
        db_pool: pg_pool,
        config: Arc::new(config),
    };

    axum::serve(listener, router::router(state)).await.unwrap();
}

async fn change_project(Path(project_id): Path<Uuid>) -> StatusCode {
    StatusCode::OK
}

async fn apply_tool(Path(project_id): Path<Uuid>) -> StatusCode {
    StatusCode::CREATED
}

async fn get_tools(Path(project_id): Path<Uuid>) -> StatusCode {
    StatusCode::OK
}

async fn delete_tool(Path(project_id): Path<Uuid>, Path(tool_id): Path<Uuid>) -> StatusCode {
    StatusCode::NO_CONTENT
}
