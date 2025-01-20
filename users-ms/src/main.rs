mod config;
mod error;
mod jwt;
mod password;
mod router;
mod user;
mod redis;

use crate::config::Config;
use crate::error::AppResult;
use clap::Parser;
use rustis::client::Client;
use rustis::client::ServerConfig::Standalone;
use sqlx::PgPool;
use std::net::IpAddr;
use std::str::FromStr;
use std::sync::Arc;
use tracing::info;

#[derive(Clone)]
struct AppState {
    pg_pool: PgPool,
    redis_client: Client,
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

    let pg_options = sqlx::postgres::PgConnectOptions::new()
        .host(&config.pg_host)
        .port(config.pg_port)
        .username(&config.pg_user)
        .password(&config.pg_password)
        .database(&config.pg_database);

    let pg_pool = PgPool::connect_with(pg_options)
        .await
        .expect("Failed to connect to Postgres");

    sqlx::migrate!()
        .run(&pg_pool)
        .await
        .expect("Failed to run migrations");

    let redis_client = Client::connect(rustis::client::Config {
        server: Standalone {
            host: config.redis_host.clone(),
            port: config.redis_port,
        },
        password: config.redis_password.clone(),
        ..Default::default()
    })
    .await
    .expect("Failed to connect to Redis");

    let state = AppState {
        pg_pool,
        redis_client,
        config: Arc::new(config),
    };

    info!("Starting server at {}:{}", addr.0, addr.1);

    tokio::select! {
        _ = axum::serve(listener, router::router(state)) => {}
        _ = tokio::signal::ctrl_c() => {}
    }
}
