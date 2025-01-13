use crate::error::Result;
use crate::tool::model::{RequestedTool, Tool};
use crate::{tool, AppState};
use axum::extract::{Path, State};
use axum::http::{header, HeaderName, HeaderValue};
use axum::routing::{get, post};
use axum::{debug_handler, Json, Router};
use uuid::Uuid;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route(
            "/projects/{project_id}/tools",
            get(get_tools).post(add_tool),
        )
        .route("/projects/{project_id}/tools/apply", post(apply_tools))
        .route(
            "/projects/{project_id}/tools/images",
            get(get_image_versions),
        )
        .route(
            "/projects/{project_id}/tools/images/{image_version_id}",
            get(download_image_version),
        )
        .with_state(state)
}

#[debug_handler]
async fn get_tools(
    Path(project_id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<Vec<Tool>>> {
    let tools = tool::controller::get_applied_tools(project_id, &state).await?;
    Ok(Json(tools))
}

#[debug_handler]
async fn add_tool(
    Path(project_id): Path<Uuid>,
    State(state): State<AppState>,
    Json(tool): Json<RequestedTool>,
) -> Result<Json<Tool>> {
    let tool = tool::controller::add_tool(project_id, tool, &state).await?;
    Ok(Json(tool))
}

#[debug_handler]
async fn apply_tools(
    Path(project_id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<()>> {
    tool::controller::apply_added_tools(project_id, &state).await?;
    // TODO: return a uuid so the client can check the status of the request, also develop that feature with websockets
    Ok(Json(()))
}

async fn get_image_versions(
    Path(project_id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<Vec<tool::model::ImageVersion>>> {
    // TODO: add the url link to each image
    let images = tool::controller::get_image_versions(project_id, &state).await?;
    Ok(Json(images))
}

async fn download_image_version(
    Path((project_id, image_version_id)): Path<(Uuid, Uuid)>,
    State(state): State<AppState>,
) -> Result<([(HeaderName, HeaderValue); 1], Vec<u8>)> {
    let image_bytes =
        tool::controller::load_image_version(project_id, image_version_id, &state).await?;

    Ok((
        [(header::CONTENT_TYPE, HeaderValue::from_static("image/png"))],
        image_bytes,
    ))
}

// TODO: delete tool endpoint which will delete the tool and all the image versions associated with it and update the position of other tools
