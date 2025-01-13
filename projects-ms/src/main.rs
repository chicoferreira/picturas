mod config;
mod error;
mod image;
mod project;
mod router;
mod tool;
mod user;

use crate::config::Config;
use crate::tool::amqp::rabbit_controller::RabbitMqController;
use crate::tool::queue::QueuedImageApplyTool;
use clap::Parser;
use dashmap::DashMap;
use sqlx::postgres::PgConnectOptions;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
struct AppState {
    db_pool: PgPool,
    config: Arc<Config>,
    rabbit_mq_controller: Arc<RabbitMqController>,
    queued_tools: Arc<DashMap<Uuid, QueuedImageApplyTool>>,
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

    let rabbit_mq_controller = RabbitMqController::new(8, &config).await;

    let state = AppState {
        db_pool: pg_pool,
        config: Arc::new(config),
        rabbit_mq_controller: Arc::new(rabbit_mq_controller),
        queued_tools: Default::default(),
    };

    let rabbit_mq_consumer = state.rabbit_mq_controller.create_consumer(&state).await;

    tokio::select! {
        _ = tool::queue::run_rabbit_mq_results_read_loop(rabbit_mq_consumer, state.clone()) => {}
        _ = axum::serve(listener, router::router(state)) => {}
    }
}

// async fn change_project(Path(project_id): Path<Uuid>) -> StatusCode {
//     StatusCode::OK
// }
//
// async fn apply_tool(Path(project_id): Path<Uuid>) -> StatusCode {
//     StatusCode::CREATED
// }
//
// async fn get_tools(Path(project_id): Path<Uuid>) -> StatusCode {
//     StatusCode::OK
// }
//
// async fn delete_tool(Path(project_id): Path<Uuid>, Path(tool_id): Path<Uuid>) -> StatusCode {
//     StatusCode::NO_CONTENT
// }
