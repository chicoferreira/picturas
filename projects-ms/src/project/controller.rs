use crate::error::{AppError, Result};
use crate::project::model::Project;
use crate::user::User;
use crate::AppState;
use chrono::Utc;
use uuid::Uuid;

pub async fn create_project(owner: User, name: String, state: AppState) -> Result<Project> {
    let now = Utc::now();
    let project = Project {
        id: Uuid::new_v4(),
        name,
        user_id: owner.uuid,
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

    Ok(project)
}

pub async fn get_project(project_id: Uuid, state: AppState) -> Result<Project> {
    let project = sqlx::query_as!(
        Project,
        "SELECT id, name, user_id, created_at, updated_at FROM projects WHERE id = $1",
        project_id
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|_| AppError::EntityNotFound)?;

    Ok(project)
}

pub async fn delete_project(project_id: Uuid, state: AppState) -> Result<()> {
    let _ = sqlx::query!("DELETE FROM projects WHERE id = $1", project_id)
        .execute(&state.db_pool)
        .await?;

    Ok(())
}
