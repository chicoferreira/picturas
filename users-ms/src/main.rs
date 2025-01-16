mod config;
mod error;
mod jwt;
mod password;
mod router;
mod user;

use crate::config::Config;
use crate::error::AppResult;
use clap::Parser;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
struct AppState {
    pg_pool: PgPool,
    config: Arc<Config>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = config::Config::parse();

    let listener = tokio::net::TcpListener::bind(&config.bind_addr)
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
        config: Arc::new(config),
    };

    axum::serve(listener, router::router(state)).await.unwrap();
}
