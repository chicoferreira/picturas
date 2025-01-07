use lapin::{ Channel, Connection};
use log::info;
use serde::Deserialize;
use serde_inline_default::serde_inline_default;
use std::sync::Arc;

mod handle;
mod message;
mod message_queue;
mod tools;
// const CONSUMER_NAMES: [&str; 8] = [
//     "crop",
//     "scale",
//     "addBorder",
//     "adjustBrightness",
//     "adjustContrast",
//     "rotate",
//     "blur",
//     "ocr",
// ];

#[derive(Debug)]
struct State {
    channel: Channel,
    config: Config,
}

#[serde_inline_default]
#[derive(Debug, Deserialize)]
struct Config {
    #[serde_inline_default("localhost".into())]
    rabbitmq_host: String,
    #[serde_inline_default("5672".into())]
    rabbitmq_port: String,
    #[serde_inline_default("guest".into())]
    rabbitmq_user: String,
    #[serde_inline_default("guest".into())]
    rabbitmq_password: String,
    picturas_available_tools: Vec<String>,
    #[serde_inline_default(42)]
    picturas_concurrent_requests: u16,
    #[serde_inline_default("picturas-multiple-tools-ms".into())]
    picturas_microservice_name: String,
    #[serde_inline_default("picturas-results".into())]
    picturas_results_exchange: String,
    #[serde_inline_default("results".into())]
    picturas_results_routing_key: String,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let config = envy::from_env::<Config>().expect("Failed to read configuration");

    let amqp_uri = format!(
        "amqp://{}:{}@{}:{}",
        config.rabbitmq_user, config.rabbitmq_password, config.rabbitmq_host, config.rabbitmq_port
    );

    let conn = Connection::connect(&amqp_uri, Default::default())
        .await
        .expect("Failed to connect to RabbitMQ");

    info!("Connected to RabbitMQ at {:?}", conn);

    let concurrent_requests = config.picturas_concurrent_requests;
    let channel = message_queue::create_channel(concurrent_requests, &conn).await;

    info!("Created channel with prefetch count of {concurrent_requests}");

    let state = Arc::new(State {
        channel: channel.clone(),
        config,
    });

    let mut join_set = tokio::task::JoinSet::new();
    for consumer_name in &state.config.picturas_available_tools {
        let state = state.clone();
        let consumer = message_queue::create_consumer(channel.clone(), consumer_name).await;

        join_set.spawn(message_queue::run_rabbitmq_queue(consumer, state.clone()));
    }

    tokio::select! {
        _ = join_set.join_next() => {}
        _ = tokio::signal::ctrl_c() => {}
    }

    let _ = conn.close(0, "bye").await;
}
