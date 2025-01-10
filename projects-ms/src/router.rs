use crate::{apply_tool, delete_tool, get_tools, image, project, AppState};
use axum::routing::{delete, get, post};
use axum::Router;

pub fn router(state: AppState) -> Router {
    Router::new().nest(
        "/api/v1",
        Router::new()
            .route("/health", get(health_check))
            .nest(
                "/projects/{project_id}",
                Router::new()
                    .route("/tools", post(apply_tool).get(get_tools))
                    .route("/tools/{tool_id}", delete(delete_tool)),
            )
            .with_state(state.clone())
            .merge(image::router::router(state.clone()))
            .merge(project::router::router(state.clone())),
    )
}

async fn health_check() -> &'static str {
    "Server is running."
}
