use crate::tool::amqp;
use crate::tool::amqp::message::ResponseStatus;
use crate::tool::amqp::rabbit_controller::{RabbitMqConsumer, RabbitMqControllerError};
use crate::tool::model::{RequestedTool, Tool};
use crate::AppState;
use chrono::Utc;
use serde_json::json;
use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug)]
pub struct QueuedImageApplyTool {
    pub uuid: Uuid,
    pub image_uri: PathBuf,
    pub missing_tools: VecDeque<RequestedTool>,
}

async fn send_request_to_rabbitmq(
    image_path: PathBuf,
    tool: &RequestedTool,
    state: &AppState,
) -> Result<Uuid, RabbitMqControllerError> {
    let uuid = Uuid::new_v4();

    let mut parameters = tool.parameters.clone();
    parameters.push((
        "inputImageURI".to_string(),
        json!(image_path.to_string_lossy().to_string()),
    ));

    let message = amqp::message::RequestMessage {
        message_id: uuid,
        timestamp: Utc::now(),
        procedure: tool.procedure.clone(),
        parameters,
    };

    state.rabbit_mq_controller.publish_request(message).await?;

    Ok(uuid)
}

pub async fn add_to_queue(
    mut queued_image_apply_tool: QueuedImageApplyTool,
    state: &AppState,
) -> Result<(), RabbitMqControllerError> {
    let Some(tool) = queued_image_apply_tool.missing_tools.pop_front() else {
        // no tool to apply
        return Ok(());
    };

    let image_path = queued_image_apply_tool.image_uri.clone();
    let message_id = send_request_to_rabbitmq(image_path, &tool, state).await?;

    state
        .queued_tools
        .insert(message_id, queued_image_apply_tool);

    Ok(())
}

pub async fn run_rabbit_mq_results_read_loop(mut consumer: RabbitMqConsumer, state: AppState) {
    while let Ok(message) = consumer.next_result_message().await {
        match message.status {
            ResponseStatus::Success { output } => {
                log::info!(
                    "Received a success response for message {}: {:?}",
                    message.message_id,
                    output
                );
                // TODO: save image
            }
            ResponseStatus::Error { error } => {
                log::error!(
                    "Received an error response for message {}: {}",
                    message.message_id,
                    error.message
                );
                // TODO: notify user
            }
        }

        let Some((_, queued_tool)) = state.queued_tools.remove(&message.message_id) else {
            log::error!(
                "Received a result for an unknown tool: {}",
                message.message_id
            );
            continue;
        };

        // if there are more tools to apply, add the tool to the queue
        if !queued_tool.missing_tools.is_empty() {
            if let Err(e) = add_to_queue(queued_tool, &state).await {
                log::error!("Failed to add tool to queue: {}", e);
            }
        }
    }
}
