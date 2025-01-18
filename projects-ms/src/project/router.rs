use crate::error::Result;
use crate::project::controller;
use crate::project::model::Project;
use crate::user::AccessTokenClaims;
use crate::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{debug_handler, Json, Router};
use serde::Deserialize;
use uuid::Uuid;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/projects", post(create_project))
        .route(
            "/projects/{project_id}",
            get(get_project).delete(delete_project),
        )
        .with_state(state)
}

#[derive(Deserialize)]
struct CreateProjectRequest {
    name: String,
}

#[debug_handler]
async fn create_project(
    State(state): State<AppState>,
    user: AccessTokenClaims,
    Json(request): Json<CreateProjectRequest>,
) -> Result<(StatusCode, Json<Project>)> {
    let project = controller::create_project(user.sub, request.name, state).await?;
    Ok((StatusCode::CREATED, Json(project)))
}

#[debug_handler]
async fn get_project(
    Path(project_id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<Project>> {
    controller::get_project(project_id, state).await.map(Json)
}

#[debug_handler]
async fn delete_project(
    Path(project_id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<StatusCode> {
    controller::delete_project(project_id, state).await?;
    Ok(StatusCode::NO_CONTENT)
}
