mod error;
mod model;
mod project;
mod router;
mod user;

use axum::extract::Path;
use axum::http::StatusCode;
use clap::Parser;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
struct AppState {
    db_pool: PgPool,
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
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let config = Config::parse();

    let pg_pool = PgPool::connect(&format!(
        "postgres://{}:{}@{}:{}/picturas",
        config.pg_user, config.pg_password, config.pg_host, config.pg_port
    ))
    .await
    .expect("Failed to connect to Postgres.");

    let state = AppState { db_pool: pg_pool };

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router::router(state)).await.unwrap();
}

async fn health_check() -> &'static str {
    "Server is running."
}

async fn get_project(Path(project_id): Path<Uuid>) -> StatusCode {
    StatusCode::OK
}

async fn change_project(Path(project_id): Path<Uuid>) -> StatusCode {
    StatusCode::OK
}

async fn delete_project(Path(project_id): Path<Uuid>) -> StatusCode {
    StatusCode::NO_CONTENT
}

async fn create_image(Path(project_id): Path<Uuid>) -> StatusCode {
    StatusCode::CREATED
}

async fn get_images(Path(project_id): Path<Uuid>) -> StatusCode {
    StatusCode::OK
}

async fn delete_image(Path(project_id): Path<Uuid>, Path(image_id): Path<Uuid>) -> StatusCode {
    StatusCode::NO_CONTENT
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
