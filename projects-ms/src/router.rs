use crate::project::controller::create_project;
use crate::{
    apply_tool, change_project, create_image, delete_image, delete_project, delete_tool,
    get_images, get_project, get_tools, health_check, AppState,
};
use axum::routing::{delete, get, post};
use axum::Router;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/projects", post(create_project))
        .nest(
            "/projects/{project_id}",
            Router::new()
                .route(
                    "/",
                    get(get_project).delete(delete_project).put(change_project),
                )
                .route("/images", post(create_image).get(get_images))
                .route("/images/{image_id}", delete(delete_image))
                .route("/tools", post(apply_tool).get(get_tools))
                .route("/tools/{tool_id}", delete(delete_tool)),
        )
        .with_state(state)
}
