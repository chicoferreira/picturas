use crate::handle::{HandleRequestError, HandleRequestResult};
use crate::message::{
    ErrorDetails, Metadata, OutputType, RequestMessage, ResponseMessage, ResponseMessageStatus,
};
use crate::{handle, State};
use anyhow::Context;
use chrono::Utc;
use futures_util::StreamExt;
use lapin::message::Delivery;
use lapin::options::{BasicAckOptions, BasicConsumeOptions, BasicPublishOptions, BasicQosOptions};
use lapin::publisher_confirm::PublisherConfirm;
use lapin::types::{FieldTable, ShortUInt};
use lapin::{BasicProperties, Channel, Connection, Consumer};
use log::{error, info};
use std::sync::Arc;
use std::time::Instant;
use uuid::Uuid;

impl From<Result<HandleRequestResult, HandleRequestError>> for ResponseMessageStatus {
    fn from(value: Result<HandleRequestResult, HandleRequestError>) -> Self {
        match value {
            Ok(request_result) => match request_result {
                HandleRequestResult::Image(path) => {
                    ResponseMessageStatus::Success(OutputType::Image { image_uri: path })
                }
                HandleRequestResult::Text(text) => {
                    ResponseMessageStatus::Success(OutputType::Text { text })
                }
            },
            Err(request_error) => {
                let code = match request_error {
                    HandleRequestError::ImageSaveError { .. } => "IMAGE_SAVE_ERROR",
                    HandleRequestError::ImageOpenError { .. } => "IMAGE_OPEN_ERROR",
                    HandleRequestError::ToolApplyError { .. } => "TOOL_APPLY_ERROR",
                    HandleRequestError::MissingOutputPath => "MISSING_OUTPUT_PATH",
                };
                ResponseMessageStatus::Error(ErrorDetails {
                    code: code.into(),
                    message: request_error.to_string(),
                })
            }
        }
    }
}

async fn publish_response(
    response: &[u8],
    state: &State,
) -> Result<PublisherConfirm, lapin::Error> {
    state
        .channel
        .basic_publish(
            &state.config.picturas_results_exchange,
            &state.config.picturas_results_routing_key,
            BasicPublishOptions::default(),
            response,
            BasicProperties::default(),
        )
        .await
}

async fn handle_rabbitmq_delivery(delivery: Delivery, state: &State) -> anyhow::Result<()> {
    let instant = Instant::now();

    let request: RequestMessage =
        serde_json::from_slice(&delivery.data).context("Failed to deserialize request")?;

    info!("Received request: {request:?}");
    let message_id = request.message_id.clone();

    let result = tokio::spawn(handle::handle_request(request)).await.unwrap();
    let response = ResponseMessage {
        message_id: Uuid::new_v4().to_string(),
        correlation_id: message_id,
        timestamp: Utc::now(),
        status: result.into(),
        metadata: Metadata {
            processing_time: instant.elapsed().as_secs_f64(),
            microservice: state.config.picturas_microservice_name.clone(),
        },
    };

    info!(
        "Processed request: {} (took {:.3}s)",
        response.message_id, response.metadata.processing_time
    );

    let response = serde_json::to_vec(&response).context("Failed to serialize response")?;
    publish_response(&response, &state)
        .await
        .context("Failed to publish response")?;

    delivery
        .ack(BasicAckOptions::default())
        .await
        .context("Failed to ack delivery")
}

pub async fn run_rabbitmq_queue(mut consumer: Consumer, state: Arc<State>) {
    info!("Consumer {consumer:?} started");

    while let Some(delivery) = consumer.next().await {
        match delivery {
            Ok(delivery) => {
                if let Err(e) = handle_rabbitmq_delivery(delivery, &state).await {
                    error!("Consumer {consumer:?} failed to handle delivery: {e}");
                }
            }
            Err(error) => {
                error!("Consumer {consumer:?} returned delivery error: {error}");
            }
        }
    }
}

pub async fn create_channel(concurrent_requests: ShortUInt, conn: &Connection) -> Channel {
    let channel = conn
        .create_channel()
        .await
        .expect("Failed to open a channel");

    channel
        .basic_qos(concurrent_requests, BasicQosOptions::default())
        .await
        .expect("Failed to set QoS");
    channel
}

pub async fn create_consumer(channel: Channel, consumer_name: &str) -> Consumer {
    channel
        .basic_consume(
            consumer_name,
            "",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .expect("Failed to register a consumer")
}
