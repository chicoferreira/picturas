use crate::tool::amqp;
use crate::tool::amqp::message::ResponseStatus;
use crate::tool::amqp::rabbit_controller::{RabbitMqConsumer, RabbitMqControllerError};
use crate::tool::model::RequestedTool;
use crate::AppState;
use chrono::Utc;
use serde_json::json;
use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use tracing::{error, info};
use uuid::Uuid;

#[derive(Debug)]
pub struct QueuedImageApplyTool {
    pub new_image_uuid: Uuid,
    pub project_folder: PathBuf,
    pub image_input_uri: PathBuf,
    pub image_output_uri: PathBuf,
    pub missing_tools: VecDeque<RequestedTool>,
}

impl QueuedImageApplyTool {
    pub fn new_generate_output_uri(
        project_folder: PathBuf,
        image_input_uri: PathBuf,
        missing_tools: VecDeque<RequestedTool>,
    ) -> Self {
        let new_image_uuid = Uuid::new_v4();
        let image_output_uri = project_folder
            .join("output")
            .join(new_image_uuid.to_string())
            .with_extension("png");

        Self {
            project_folder,
            new_image_uuid,
            image_input_uri,
            image_output_uri,
            missing_tools,
        }
    }
}

async fn send_request_to_rabbitmq(
    image_input_path: &Path,
    image_output_path: &Path,
    tool: &RequestedTool,
    state: &AppState,
) -> Result<Uuid, RabbitMqControllerError> {
    let message_uuid = Uuid::new_v4();

    let mut parameters = tool.parameters.clone();
    parameters.insert(
        "inputImageURI".to_string(),
        json!(image_input_path.to_string_lossy().to_string()),
    );
    parameters.insert(
        "outputImageURI".to_string(),
        json!(image_output_path.to_string_lossy().to_string()),
    );

    let message = amqp::message::RequestMessage {
        message_id: message_uuid,
        timestamp: Utc::now(),
        procedure: tool.procedure.clone(),
        parameters,
    };

    state.rabbit_mq_controller.publish_request(message).await?;

    Ok(message_uuid)
}

pub async fn add_to_queue(
    mut queued_image_apply_tool: QueuedImageApplyTool,
    state: &AppState,
) -> Result<(), RabbitMqControllerError> {
    let Some(tool) = queued_image_apply_tool.missing_tools.pop_front() else {
        // no tool to apply
        return Ok(());
    };

    let image_input_path = &queued_image_apply_tool.image_input_uri;
    let image_output_path = &queued_image_apply_tool.image_output_uri;

    let message_id =
        send_request_to_rabbitmq(image_input_path, image_output_path, &tool, state).await?;

    state
        .queued_tools
        .insert(message_id, queued_image_apply_tool);

    Ok(())
}

pub async fn run_rabbit_mq_results_read_loop(mut consumer: RabbitMqConsumer, state: AppState) {
    while let Ok(message) = consumer.next_result_message().await {
        let Some((_, queued_tool)) = state.queued_tools.remove(&message.correlation_id) else {
            error!(
                "Received a result for an unknown tool: {}",
                message.correlation_id
            );
            continue;
        };

        match message.status {
            ResponseStatus::Success { output } => {
                info!(
                    "Received a success response for message {}: {:?}",
                    message.message_id, output
                );
                // TODO: save image
            }
            ResponseStatus::Error { error } => {
                error!(
                    "Received an error response for message {}: {}",
                    message.message_id, error.message
                );
            }
        }

        // if there are more tools to apply, add the tool to the queue
        if !queued_tool.missing_tools.is_empty() {
            let queued_tool = QueuedImageApplyTool::new_generate_output_uri(
                queued_tool.project_folder,
                queued_tool.image_output_uri, // now the new input will be the output of the previous tool
                queued_tool.missing_tools,
            );

            if let Err(e) = add_to_queue(queued_tool, &state).await {
                error!("Failed to add tool to queue: {}", e);
            }
        }
    }
}
