use crate::tool::amqp::message::OutputType::Text;
use crate::tool::amqp::message::ResponseStatus;
use crate::tool::amqp::rabbit_controller::{RabbitMqConsumer, RabbitMqControllerError};
use crate::tool::controller::ImageVersionWithUrl;
use crate::tool::model::{ImageVersion, RequestedTool};
use crate::tool::{amqp, controller, websocket};
use crate::{config, AppState};
use chrono::Utc;
use serde::Serialize;
use serde_json::json;
use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use tracing::{error, info};
use uuid::Uuid;

#[derive(Debug)]
pub struct QueuedImageApplyTool {
    pub new_image_uuid: Uuid,
    pub original_image_uuid: Uuid,
    pub project_id: Uuid,
    pub user_id: Uuid,
    pub image_input_uri: PathBuf,
    pub image_output_uri: PathBuf,
    pub missing_tools: VecDeque<(Uuid, RequestedTool)>, // tool_uuid, tool
}

impl QueuedImageApplyTool {
    pub fn new_generate_output_uri(
        project_id: Uuid,
        user_id: Uuid,
        original_image_uuid: Uuid,
        image_input_uri: PathBuf,
        missing_tools: VecDeque<(Uuid, RequestedTool)>,
        state: &AppState,
    ) -> Self {
        let new_image_uuid = Uuid::new_v4();

        let image_output_uri = config::generate_image_version_output_uri(
            project_id,
            original_image_uuid,
            new_image_uuid,
            state,
        );

        Self {
            new_image_uuid,
            image_input_uri,
            image_output_uri,
            missing_tools,
            original_image_uuid,
            project_id,
            user_id,
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
    let Some((tool_uuid, requested_tool)) = queued_image_apply_tool.missing_tools.pop_front()
    else {
        // no tool to apply
        return Ok(());
    };

    let image_input_path = &queued_image_apply_tool.image_input_uri;
    let image_output_path = &queued_image_apply_tool.image_output_uri;

    let message_id =
        send_request_to_rabbitmq(image_input_path, image_output_path, &requested_tool, state)
            .await?;

    state
        .queued_tools
        .insert(message_id, (tool_uuid, queued_image_apply_tool));

    Ok(())
}

pub async fn run_rabbit_mq_results_read_loop(mut consumer: RabbitMqConsumer, state: AppState) {
    while let Ok(message) = consumer.next_result_message().await {
        let Some((_, (tool_uuid, queued_tool))) =
            state.queued_tools.remove(&message.correlation_id)
        else {
            error!(
                "Received a result for an unknown tool: {}",
                message.correlation_id
            );
            continue;
        };

        match message.status {
            ResponseStatus::Success { output } => {
                info!(message = ?message.message_id, ?output, "Received a success response");

                let image_version = ImageVersion {
                    id: queued_tool.new_image_uuid,
                    original_image_id: queued_tool.original_image_uuid,
                    project_id: queued_tool.project_id,
                    tool_id: tool_uuid,
                    text_result: (output.kind == Text).then_some(output.text).flatten(),
                    created_at: Utc::now(),
                };

                // save the image version to the database
                if let Err(e) = controller::save_image_version(&image_version, &state).await {
                    error!("Failed to save image version to the database: {}", e);
                }

                let notification = ImageVersionWithUrl::from_image_version(image_version, &state);

                if let Err(err) = websocket::send_ws_message(
                    &state,
                    queued_tool.project_id,
                    queued_tool.user_id,
                    notification,
                )
                .await
                {
                    error!("Failed to send message to websocket: {}", err);
                }

                // if there are more tools to apply, add the tool to the queue
                if !queued_tool.missing_tools.is_empty() {
                    let queued_tool = QueuedImageApplyTool::new_generate_output_uri(
                        queued_tool.project_id,
                        queued_tool.user_id,
                        queued_tool.original_image_uuid,
                        queued_tool.image_output_uri, // now the new input will be the output of the previous tool
                        queued_tool.missing_tools,
                        &state,
                    );

                    if let Err(e) = add_to_queue(queued_tool, &state).await {
                        error!("Failed to add tool to queue: {}", e);
                    }
                }
            }
            ResponseStatus::Error { error } => {
                info!(message = ?message.message_id, ?error, "Received a error response");

                if let Err(err) = websocket::send_ws_message(
                    &state,
                    queued_tool.project_id,
                    queued_tool.user_id,
                    json!({
                        "error": error,
                    }),
                )
                .await
                {
                    error!("Failed to send message to websocket: {}", err);
                }
                continue; // we can't apply the next tool if the current one failed
            }
        }
    }
}
