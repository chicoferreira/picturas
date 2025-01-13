use crate::{image, project, tool, AppState};
use axum::routing::get;
use axum::Router;

pub fn router(state: AppState) -> Router {
    Router::new().nest(
        "/api/v1",
        Router::new()
            .route("/health", get(health_check))
            .merge(tool::router::router(state.clone()))
            .merge(image::router::router(state.clone()))
            .merge(project::router::router(state.clone())),
    )
}

async fn health_check() -> &'static str {
    "Server is running."
}
