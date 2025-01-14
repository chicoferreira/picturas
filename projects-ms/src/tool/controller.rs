use crate::error::Result;
use crate::tool::model::{ImageVersion, RequestedTool, Tool};
use crate::tool::queue;
use crate::tool::queue::QueuedImageApplyTool;
use crate::{config, image, tool, AppState};
use serde::{Deserialize, Serialize};
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

pub async fn get_image_versions(project_id: Uuid, state: &AppState) -> Result<Vec<ImageVersion>> {
    let images = sqlx::query_as!(
        ImageVersion,
        "SELECT id, original_image_id, project_id, tool_id, text_result, created_at FROM image_versions WHERE project_id = $1",
        project_id
    )
        .fetch_all(&state.db_pool)
        .await?;

    Ok(images)
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

async fn delete_image_versions(project_uuid: Uuid, state: &AppState) -> Result<()> {
    let delete_files = tokio::fs::remove_dir_all(config::generate_image_version_folder_uri(
        project_uuid,
        state,
    ));

    let delete_sql = sqlx::query!(
        "DELETE FROM image_versions WHERE project_id = $1",
        project_uuid
    )
    .execute(&state.db_pool);

    let (_delete_files, delete_sql) = tokio::join!(delete_files, delete_sql);

    delete_sql?;

    Ok(())
}

pub async fn update_tools(
    project_uuid: Uuid,
    tools: Vec<RequestedTool>,
    state: &AppState,
) -> Result<Vec<Tool>> {
    delete_image_versions(project_uuid, state).await?;

    sqlx::query!("DELETE FROM tools WHERE project_id = $1", project_uuid)
        .execute(&state.db_pool)
        .await?;

    let mut result = vec![];

    for (position, requested_tool) in tools.into_iter().enumerate() {
        let tool = Tool {
            id: Uuid::new_v4(),
            project_id: project_uuid,
            position: position as i32 + 1,
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

        result.push(tool);
    }

    Ok(result)
}

pub async fn apply_added_tools(
    project_uuid: Uuid,
    user_uuid: Uuid,
    state: &AppState,
) -> Result<()> {
    delete_image_versions(project_uuid, state).await?;

    let (images, tools) = tokio::join!(
        image::controller::get_original_images(project_uuid, state),
        get_applied_tools(project_uuid, state)
    );

    let requested_tools: VecDeque<(Uuid, RequestedTool)> = tools?
        .into_iter()
        .filter_map(|tool| Some((tool.id, tool.try_into().ok()?)))
        .collect();

    for image in images? {
        let image_input_uri = image.get_uri(state);

        let queued_image_apply_tool = QueuedImageApplyTool::new_generate_output_uri(
            project_uuid,
            user_uuid,
            image.id,
            image_input_uri,
            requested_tools.clone(),
            state,
        );

        debug!(queued_image_apply_tool = ?queued_image_apply_tool, "Queued image apply tool");
        queue::add_to_queue(queued_image_apply_tool, state).await?;
    }

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageVersionWithUrl {
    #[serde(flatten)]
    image_version: ImageVersion,
    url: String,
}

impl ImageVersionWithUrl {
    pub fn from_image_version(image_version: ImageVersion, state: &AppState) -> Self {
        let url = format!(
            "{}/api/v1/projects/{}/tools/images/{}",
            state.config.picturas_public_url, image_version.project_id, image_version.id
        );
        Self { url, image_version }
    }
}

pub async fn save_image_version(image_version: &ImageVersion, state: &AppState) -> Result<()> {
    sqlx::query!(
        "INSERT INTO image_versions (id, original_image_id, project_id, tool_id, text_result, created_at) VALUES ($1, $2, $3, $4, $5, $6)",
        image_version.id,
        image_version.original_image_id,
        image_version.project_id,
        image_version.tool_id,
        image_version.text_result,
        image_version.created_at
    )
        .execute(&state.db_pool)
        .await?;

    Ok(())
}

pub async fn load_image_version(
    project_id: Uuid,
    image_version_uuid: Uuid,
    state: &AppState,
) -> Result<Vec<u8>> {
    let image_version = sqlx::query_as!(
        ImageVersion,
        "SELECT id, original_image_id, project_id, tool_id, text_result, created_at FROM image_versions WHERE project_id = $1 AND id = $2",
        project_id,
        image_version_uuid
    )
        .fetch_one(&state.db_pool)
        .await?;

    let image_path = image_version.get_uri(state);
    let image_data = tokio::fs::read(image_path).await?;

    Ok(image_data)
}
