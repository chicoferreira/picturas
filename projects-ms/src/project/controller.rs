use crate::error::Result;
use crate::project::model::Project;
use crate::user::User;
use crate::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{debug_handler, Json};
use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateProjectRequest {
    name: String,
}

#[debug_handler]
pub async fn create_project(
    State(state): State<AppState>,
    user: User,
    Json(request): Json<CreateProjectRequest>,
) -> Result<(StatusCode, Json<Project>)> {
    let now = Utc::now();
    let project = Project {
        id: Uuid::new_v4(),
        name: request.name,
        user_id: user.uuid,
        created_at: now,
        updated_at: now,
    };

    let _ = sqlx::query!(
        "INSERT INTO projects (id, name, user_id, created_at, updated_at) VALUES ($1, $2, $3, $4, $5)",
        project.id,
        project.name,
        project.user_id,
        project.created_at,
        project.updated_at
    )
        .execute(&state.db_pool)
        .await?;

    Ok((StatusCode::CREATED, Json(project)))
}

#[debug_handler]
pub async fn get_project(
    Path(project_id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<Project>> {
    let project = sqlx::query_as!(
        Project,
        "SELECT id, name, user_id, created_at, updated_at FROM projects WHERE id = $1",
        project_id
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|_| crate::error::Error::EntityNotFound)?;

    Ok(Json(project))
}
