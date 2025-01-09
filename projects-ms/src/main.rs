mod project;

use axum::extract::Path;
use axum::routing::delete;
use axum::{
    http::StatusCode,
    routing::{get, post},
    Router,
};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    env_logger::init();

    let app = Router::new()
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
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "Hello, World!"
}

async fn create_project() -> StatusCode {
    StatusCode::CREATED
}

async fn get_project(Path(project_id): Path<Uuid>) -> StatusCode {
    StatusCode::OK
}

async fn change_project(Path(project_id): Path<Uuid>) -> StatusCode {
    StatusCode::OK
}

async fn delete_project(Path(project_id): Path<Uuid>) -> StatusCode {
    StatusCode::NO_CONTENT
}

async fn create_image(Path(project_id): Path<Uuid>) -> StatusCode {
    StatusCode::CREATED
}

async fn get_images(Path(project_id): Path<Uuid>) -> StatusCode {
    StatusCode::OK
}

async fn delete_image(Path(project_id): Path<Uuid>, Path(image_id): Path<Uuid>) -> StatusCode {
    StatusCode::NO_CONTENT
}

async fn apply_tool(Path(project_id): Path<Uuid>) -> StatusCode {
    StatusCode::CREATED
}

async fn get_tools(Path(project_id): Path<Uuid>) -> StatusCode {
    StatusCode::OK
}

async fn delete_tool(Path(project_id): Path<Uuid>, Path(tool_id): Path<Uuid>) -> StatusCode {
    StatusCode::NO_CONTENT
}
