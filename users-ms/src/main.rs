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
use std::net::IpAddr;
use std::str::FromStr;
use std::sync::Arc;
use tracing::info;

#[derive(Clone)]
struct AppState {
    pg_pool: PgPool,
    config: Arc<Config>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = config::Config::parse();

    let addr = (IpAddr::from_str(&config.bind_ip).unwrap(), config.bind_port);
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to port");

    let options = sqlx::postgres::PgConnectOptions::new()
        .host(&config.pg_host)
        .port(config.pg_port)
        .username(&config.pg_user)
        .password(&config.pg_password)
        .database(&config.pg_database);

    let pool = PgPool::connect_with(options)
        .await
        .expect("Failed to connect to Postgres");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let state = AppState {
        pg_pool: pool,
        config: Arc::new(config),
    };

    info!("Starting server at {}:{}", addr.0, addr.1);

    tokio::select! {
        _ = axum::serve(listener, router::router(state)) => {}
        _ = tokio::signal::ctrl_c() => {}
    }
}
