use crate::tool::amqp::message::{RequestMessage, ResponseMessage};
use crate::{AppState, Config};
use futures_util::StreamExt;
use lapin::options::{
    BasicPublishOptions, ExchangeDeclareOptions, QueueBindOptions, QueueDeclareOptions,
};
use lapin::types::FieldTable;
use lapin::{BasicProperties, Channel, Connection, Consumer, ExchangeKind};
use std::collections::HashMap;
use thiserror::Error;

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
        let consumer = create_results_consumer(
            &self.channel,
            &state.config.rabbitmq_results_exchange,
            &state.config.rabbitmq_results_routing_key,
        )
        .await;

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
            Some(Ok(delivery)) => Ok(serde_json::from_slice(&delivery.data)?),
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

    let connection = Connection::connect(&amqp_uri, Default::default())
        .await
        .expect("Failed to connect to RabbitMQ");

    let channel = connection
        .create_channel()
        .await
        .expect("Failed to open a channel");

    channel
        .basic_qos(concurrent_requests, Default::default())
        .await
        .expect("Failed to set QoS");

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
