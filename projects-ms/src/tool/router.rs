use crate::error::Result;
use crate::tool::controller::ImageVersionWithUrl;
use crate::tool::model::{RequestedTool, Tool};
use crate::tool::websocket;
use crate::user::AccessTokenClaims;
use crate::{tool, AppState};
use axum::extract::{Path, State};
use axum::http::{header, HeaderName, HeaderValue};
use axum::routing::{get, post};
use axum::{debug_handler, Json, Router};
use serde_json::{json, Value};
use uuid::Uuid;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route(
            "/projects/{project_id}/tools",
            get(get_tools).post(add_tool).put(put_tools),
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
        .with_state(state.clone())
        .merge(websocket::router(state))
}

#[debug_handler]
async fn get_tools(
    Path(project_id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<Vec<Tool>>> {
    tool::controller::get_applied_tools(project_id, &state)
        .await
        .map(Json)
}

#[debug_handler]
async fn add_tool(
    Path(project_id): Path<Uuid>,
    State(state): State<AppState>,
    Json(tool): Json<RequestedTool>,
) -> Result<Json<Tool>> {
    tool::controller::add_tool(project_id, tool, &state)
        .await
        .map(Json)
}

#[debug_handler]
async fn put_tools(
    Path(project_id): Path<Uuid>,
    State(state): State<AppState>,
    Json(tools): Json<Vec<RequestedTool>>,
) -> Result<Json<Vec<Tool>>> {
    tool::controller::update_tools(project_id, tools, &state)
        .await
        .map(Json)
}

#[debug_handler]
async fn apply_tools(
    Path(project_id): Path<Uuid>,
    user: AccessTokenClaims,
    State(state): State<AppState>,
) -> Result<Json<Value>> {
    tool::controller::apply_added_tools(project_id, user.sub, &state).await?;
    Ok(Json(json!({
        "message": "Hook to websocket to get realtime results",
    })))
}

#[debug_handler]
async fn get_image_versions(
    Path(project_id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<Vec<ImageVersionWithUrl>>> {
    let images = tool::controller::get_image_versions(project_id, &state)
        .await?
        .into_iter()
        .map(|image_version| ImageVersionWithUrl::from_image_version(image_version, &state))
        .collect();

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
