use crate::error::{AppError, Result};
use crate::project::model::Project;
use crate::AppState;
use chrono::Utc;
use tracing::info;
use uuid::Uuid;

pub async fn create_project(owner: Uuid, name: String, state: AppState) -> Result<Project> {
    info!("Creating project with name: {}", name);
    let now = Utc::now();
    let project = Project {
        id: Uuid::new_v4(),
        name,
        user_id: owner,
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

    info!("Project created with ID: {}", project.id);
    Ok(project)
}

pub async fn get_project(project_id: Uuid, state: AppState) -> Result<Project> {
    info!("Fetching project with ID: {}", project_id);
    let project = sqlx::query_as!(
        Project,
        "SELECT id, name, user_id, created_at, updated_at FROM projects WHERE id = $1",
        project_id
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|_| AppError::EntityNotFound)?;

    info!("Fetched project with ID: {}", project.id);
    Ok(project)
}

pub async fn delete_project(project_id: Uuid, state: AppState) -> Result<()> {
    info!("Deleting project with ID: {}", project_id);
    let _ = sqlx::query!("DELETE FROM projects WHERE id = $1", project_id)
        .execute(&state.db_pool)
        .await?;

    info!("Deleted project with ID: {}", project_id);
    Ok(())
}
