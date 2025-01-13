use crate::error::Result;
use crate::tool::model::{ImageVersion, RequestedTool, Tool};
use crate::tool::queue;
use crate::tool::queue::QueuedImageApplyTool;
use crate::{image, AppState};
use std::collections::VecDeque;
use tracing::{debug, info};
use uuid::Uuid;

pub async fn get_applied_tools(project_uuid: Uuid, state: &AppState) -> Result<Vec<Tool>> {
    let tools = sqlx::query_as!(
        Tool,
        "SELECT id, position, procedure, parameters, project_id FROM tools WHERE project_id = $1 ORDER BY position ASC",
        project_uuid
    )
        .fetch_all(&state.db_pool)
        .await?;

    Ok(tools)
}

async fn get_last_applied_tool(project_uuid: Uuid, state: &AppState) -> Result<Option<Tool>> {
    let tool = sqlx::query_as!(
        Tool,
        "SELECT id, position, procedure, parameters, project_id FROM tools WHERE project_id = $1 ORDER BY position DESC LIMIT 1",
        project_uuid
    )
        .fetch_optional(&state.db_pool)
        .await?;

    Ok(tool)
}

pub async fn get_image_versions(tool_uuid: Uuid, state: &AppState) -> Result<Vec<ImageVersion>> {
    let images = sqlx::query_as!(
        ImageVersion,
        "SELECT id, image_id, tool_id, text_result, created_at FROM image_versions WHERE tool_id = $1",
        tool_uuid
    )
        .fetch_all(&state.db_pool)
        .await?;

    Ok(images)
}

async fn insert_tool(tool: &Tool, state: &AppState) -> Result<()> {
    sqlx::query!(
        "INSERT INTO tools (id, project_id, position, procedure, parameters) VALUES ($1, $2, $3, $4, $5)",
        tool.id,
        tool.project_id,
        tool.position,
        tool.procedure,
        tool.parameters
    )
        .execute(&state.db_pool)
        .await?;

    Ok(())
}

pub async fn add_tool(
    project_uuid: Uuid,
    requested_tool: RequestedTool,
    state: &AppState,
) -> Result<Tool> {
    info!(
        tool = ?requested_tool,
        "Adding tool to project: {}", project_uuid
    );
    let last_applied_tool = get_last_applied_tool(project_uuid, state).await?;

    let last_position = match &last_applied_tool {
        None => 0,
        Some(last_applied_tool) => last_applied_tool.position,
    };

    debug!(last_position, "Last position");

    let tool = Tool {
        id: Uuid::new_v4(),
        project_id: project_uuid,
        position: last_position + 1,
        procedure: requested_tool.procedure.clone(),
        parameters: serde_json::to_value(requested_tool.parameters.clone()).unwrap(),
    };

    sqlx::query!(
        "INSERT INTO tools (id, project_id, position, procedure, parameters) VALUES ($1, $2, $3, $4, $5)",
        tool.id,
        tool.project_id,
        tool.position,
        tool.procedure,
        tool.parameters
    )
        .execute(&state.db_pool)
        .await?;

    Ok(tool)
}

pub async fn apply_added_tools(project_uuid: Uuid, state: &AppState) -> Result<()> {
    let (images, tools) = tokio::join!(
        image::controller::get_original_images(project_uuid, state),
        get_applied_tools(project_uuid, state)
    );

    let requested_tools: VecDeque<RequestedTool> = tools?
        .into_iter()
        .filter_map(|tool| tool.try_into().ok())
        .collect();

    for image in images? {
        let image_input_uri = image.get_uri(state);
        let project_folder = image_input_uri.parent().unwrap().to_path_buf();

        let queued_image_apply_tool = QueuedImageApplyTool::new_generate_output_uri(
            project_folder,
            image_input_uri,
            requested_tools.clone(),
        );

        debug!(queued_image_apply_tool = ?queued_image_apply_tool, "Queued image apply tool");
        queue::add_to_queue(queued_image_apply_tool, state).await?;
    }

    Ok(())
}
