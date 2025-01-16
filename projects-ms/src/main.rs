mod config;
mod error;
mod image;
mod project;
mod router;
mod state;
mod tool;
mod user;

use crate::config::Config;
use crate::state::AppState;
use crate::tool::amqp::rabbit_controller::RabbitMqController;
use clap::Parser;
use sqlx::postgres::PgConnectOptions;
use sqlx::PgPool;
use std::net::IpAddr;
use std::str::FromStr;
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = Config::parse();

    let conn = PgConnectOptions::new()
        .host(&config.pg_host)
        .port(config.pg_port)
        .username(&config.pg_user)
        .password(&config.pg_password)
        .database(&config.pg_database);

    let pg_pool = PgPool::connect_with(conn)
        .await
        .expect("Failed to connect to Postgres.");

    let bind_address = (IpAddr::from_str(&config.bind_ip).unwrap(), config.bind_port);
    let listener = tokio::net::TcpListener::bind(bind_address).await.unwrap();

    let rabbit_mq_controller = RabbitMqController::new(8, &config).await;

    let state = AppState {
        db_pool: pg_pool,
        config: Arc::new(config),
        rabbit_mq_controller: Arc::new(rabbit_mq_controller),
        queued_tools: Default::default(),
        connected_ws_clients: Default::default(),
    };

    let rabbit_mq_consumer = state.rabbit_mq_controller.create_consumer(&state).await;

    info!("Starting server at {}:{}", bind_address.0, bind_address.1);

    tokio::select! {
        _ = tool::queue::run_rabbit_mq_results_read_loop(rabbit_mq_consumer, state.clone()) => {}
        _ = axum::serve(listener, router::router(state).layer(TraceLayer::new_for_http())) => {}
    }
}
