use crate::error::Result;
use crate::tool::model::{ImageVersion, RequestedTool, Tool};
use crate::tool::queue;
use crate::tool::queue::QueuedImageApplyTool;
use crate::{image, AppState};
use std::collections::VecDeque;
use std::path::PathBuf;
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
) -> Result<()> {
    let last_applied_tool = get_last_applied_tool(project_uuid, state).await?;

    let last_position = match &last_applied_tool {
        None => 0,
        Some(last_applied_tool) => last_applied_tool.position,
    };

    let images_uri_to_apply_tool: Vec<PathBuf> = match last_applied_tool {
        None => {
            // If there is no tool applied yet, we need to apply the tool to the original images
            let images = image::controller::get_original_images(project_uuid, state).await?;
            images.iter().map(|image| image.get_uri(state)).collect()
        }
        Some(last_applied_tool) => {
            // If there is a tool applied, we need to apply the tool to the images created by the last tool
            let last_image_versions = get_image_versions(last_applied_tool.id, state).await?;
            last_image_versions
                .iter()
                .map(|image_version| image_version.get_uri(state))
                .collect()
        }
    };

    let tool = Tool {
        id: Uuid::new_v4(),
        project_id: project_uuid,
        position: last_position + 1,
        procedure: requested_tool.procedure.clone(),
        parameters: serde_json::to_value(requested_tool.parameters.clone()).unwrap(),
    };

    insert_tool(&tool, state).await?;

    for image in images_uri_to_apply_tool {
        let queued_image_apply_tool = QueuedImageApplyTool {
            uuid: Uuid::new_v4(),
            image_uri: image,
            missing_tools: VecDeque::from(vec![requested_tool.clone()]),
        };

        queue::add_to_queue(queued_image_apply_tool, state).await?;
    }

    Ok(())
}
