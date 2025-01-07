use crate::handle::{HandleRequestError, HandleRequestResult};
use crate::message::{
    ErrorDetails, Metadata, OutputType, RequestMessage, ResponseMessage, ResponseMessageStatus,
};
use chrono::{DateTime, Utc};
use futures_util::StreamExt;
use lapin::{options::*, types::FieldTable, BasicProperties, Connection};
use log::info;
use std::sync::Arc;
use std::time::Instant;
use uuid::Uuid;

mod handle;
mod message;
mod tools;

const CONCURRENT_REQUESTS: u16 = 4;
const CONSUMER_NAMES: [&str; 8] = [
    "crop",
    "scale",
    "addBorder",
    "adjustBrightness",
    "adjustContrast",
    "rotate",
    "blur",
    "ocr",
];

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

#[tokio::main]
async fn main() {
    let addr = std::env::var("RABBITMQ_GOST").unwrap_or_else(|_| "localhost".into());

    let conn = Connection::connect(&addr, Default::default())
        .await
        .expect("Failed to connect to RabbitMQ");

    info!("Connected to RabbitMQ at {}", addr);

    let channel = Arc::new(
        conn.create_channel()
            .await
            .expect("Failed to open a channel"),
    );

    channel
        .basic_qos(CONCURRENT_REQUESTS, BasicQosOptions::default())
        .await
        .expect("Failed to set QoS");

    let mut join_set = tokio::task::JoinSet::new();
    for consumer_name in CONSUMER_NAMES {
        let channel = Arc::clone(&channel);
        let mut consumer = channel
            .basic_consume(
                consumer_name,
                "tools_consumer",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .expect("Failed to register a consumer");

        join_set.spawn(async move {
            info!("Consumer {consumer:?} started");

            while let Some(delivery) = consumer.next().await {
                let instant = Instant::now();
                let delivery = delivery.expect("Failed to consume message");

                let request: RequestMessage =
                    serde_json::from_slice(&delivery.data).expect("Failed to parse request");

                info!("Received request: {:?}", request);
                let message_id = request.message_id.clone();

                let result = tokio::spawn(handle::handle_request(request)).await.unwrap();
                let response = ResponseMessage {
                    message_id: Uuid::new_v4().to_string(),
                    correlation_id: message_id,
                    timestamp: Utc::now(),
                    status: result.into(),
                    metadata: Metadata {
                        processing_time: instant.elapsed().as_secs_f64(),
                        // TODO: parameterize this in env var
                        microservice: "tools-microservice".to_string(),
                    },
                };

                let response = serde_json::to_vec(&response).expect("Failed to serialize response");

                channel
                    .basic_publish(
                        // TODO: parameterize this in var
                        "",
                        "tools_responses",
                        BasicPublishOptions::default(),
                        &response,
                        BasicProperties::default(),
                    )
                    .await
                    .expect("Failed to publish response");

                delivery
                    .ack(BasicAckOptions::default())
                    .await
                    .expect("Failed to ack message");
            }
        });
    }

    tokio::select! {
        _ = join_set.join_next() => {}
        _ = tokio::signal::ctrl_c() => {}
    }
}
