use crate::tool::amqp::message::{RequestMessage, ResponseMessage};
use crate::{AppState, Config};
use futures_util::StreamExt;
use lapin::options::{
    BasicAckOptions, BasicPublishOptions, ExchangeDeclareOptions, QueueBindOptions,
    QueueDeclareOptions,
};
use lapin::types::FieldTable;
use lapin::{BasicProperties, Channel, Connection, Consumer, ExchangeKind};
use std::collections::HashMap;
use thiserror::Error;
use tracing::info;

#[derive(Debug, Clone)]
pub struct ToolQueue {
    pub name: String,
    pub routing_key: String,
}

pub struct RabbitMqController {
    channel: Channel,
    exchange: String,
    procedure_routing_key_map: HashMap<String, String>,
}

impl RabbitMqController {
    pub async fn new(concurrent_requests: u16, config: &Config) -> Self {
        let (_connection, channel) = connect(concurrent_requests, config).await;

        let exchange = &config.rabbitmq_results_exchange;

        let tool_queues = &config.picturas_available_tools;
        setup_exchange_and_queues(&channel, exchange, tool_queues).await;

        let procudure_routing_key_map = tool_queues
            .iter()
            .map(|tool| (tool.name.clone(), tool.routing_key.clone()))
            .collect();

        Self {
            channel,
            exchange: exchange.clone(),
            procedure_routing_key_map: procudure_routing_key_map,
        }
    }

    pub async fn create_consumer(&self, state: &AppState) -> RabbitMqConsumer {
        let exchange = &state.config.rabbitmq_results_exchange;
        let routing_key = &state.config.rabbitmq_results_routing_key;
        let consumer = create_results_consumer(&self.channel, exchange, routing_key).await;

        info!(exchange, routing_key, "Created consumer");

        RabbitMqConsumer { consumer }
    }

    pub async fn publish_request(
        &self,
        request: RequestMessage,
    ) -> Result<(), RabbitMqControllerError> {
        let procedure = request.procedure.clone();
        let routing_key = self
            .procedure_routing_key_map
            .get(&procedure)
            .ok_or(RabbitMqControllerError::UnknownToolProcedure(procedure))?;

        self.channel
            .basic_publish(
                &self.exchange,
                routing_key,
                BasicPublishOptions::default(),
                &serde_json::to_vec(&request)?,
                BasicProperties::default(),
            )
            .await?;
        Ok(())
    }
}

pub struct RabbitMqConsumer {
    consumer: Consumer,
}

impl RabbitMqConsumer {
    pub async fn next_result_message(
        &mut self,
    ) -> Result<ResponseMessage, RabbitMqControllerError> {
        let delivery = self.consumer.next().await;
        match delivery {
            Some(Ok(delivery)) => {
                let message = serde_json::from_slice(&delivery.data)?;
                delivery.ack(BasicAckOptions::default()).await?;
                Ok(message)
            }
            Some(Err(error)) => Err(RabbitMqControllerError::LapinError(error)),
            None => Err(RabbitMqControllerError::EmptyIterator),
        }
    }
}

async fn connect(concurrent_requests: u16, config: &Config) -> (Connection, Channel) {
    let amqp_uri = format!(
        "amqp://{}:{}@{}:{}",
        config.rabbitmq_user, config.rabbitmq_password, config.rabbitmq_host, config.rabbitmq_port
    );

    info!(uri = amqp_uri, "Connecting to RabbitMQ");

    let connection = Connection::connect(&amqp_uri, Default::default())
        .await
        .expect("Failed to connect to RabbitMQ");

    info!("Connected to RabbitMQ");

    let channel = connection
        .create_channel()
        .await
        .expect("Failed to open a channel");

    info!("Created channel");

    channel
        .basic_qos(concurrent_requests, Default::default())
        .await
        .expect("Failed to set QoS");

    info!(concurrent_requests, "Set QoS");

    (connection, channel)
}

async fn setup_exchange_and_queues(channel: &Channel, exchange: &str, tool_queues: &[ToolQueue]) {
    channel
        .exchange_declare(
            exchange,
            ExchangeKind::Direct,
            ExchangeDeclareOptions {
                durable: true,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await
        .expect("Failed to declare the exchange");

    info!(exchange, "Declared exchange");

    for tool in tool_queues {
        channel
            .queue_declare(
                &tool.name,
                QueueDeclareOptions {
                    durable: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await
            .expect("Failed to declare the queue");

        info!(tool.name, "Declared durable queue");

        channel
            .queue_bind(
                &tool.name,
                exchange,
                &tool.routing_key,
                QueueBindOptions::default(),
                FieldTable::default(),
            )
            .await
            .expect("Failed to bind queue to exchange");

        info!(tool.name, tool.routing_key, "Bound queue to exchange");
    }
}

async fn create_results_consumer(
    channel: &Channel,
    exchange: &str,
    results_routing_key: &str,
) -> Consumer {
    const RESULTS_QUEUE: &str = "results";
    channel
        .queue_declare(
            RESULTS_QUEUE,
            QueueDeclareOptions {
                durable: true,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await
        .expect("Failed to declare the results queue");

    info!(RESULTS_QUEUE, "Declared results queue");

    channel
        .queue_bind(
            RESULTS_QUEUE,
            exchange,
            results_routing_key,
            QueueBindOptions::default(),
            FieldTable::default(),
        )
        .await
        .expect("Failed to bind results queue to exchange");

    info!(
        RESULTS_QUEUE,
        exchange, results_routing_key, "Bound results queue to exchange"
    );

    channel
        .basic_consume(RESULTS_QUEUE, "", Default::default(), FieldTable::default())
        .await
        .expect("Failed to register a consumer")
}

#[derive(Debug, Error)]
pub enum RabbitMqControllerError {
    #[error("Failed to serialize or deserialize JSON: {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("Failed to publish message: {0}")]
    LapinError(#[from] lapin::Error),
    #[error("Iterator is empty")]
    EmptyIterator,
    #[error("Unknown tool procedure: {0}")]
    UnknownToolProcedure(String),
}
