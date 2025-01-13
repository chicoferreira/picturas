use crate::error::Result;
use crate::tool::model::{RequestedTool, Tool};
use crate::{tool, AppState};
use axum::extract::{Path, State};
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
    // TODO: return a uuid so the client can check the status of the request
    Ok(Json(()))
}
