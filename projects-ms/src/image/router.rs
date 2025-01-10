use crate::error::{AppError, Result};
use crate::image::controller;
use crate::image::model::Image;
use crate::AppState;
use anyhow::{anyhow, Context};
use axum::extract::{DefaultBodyLimit, Multipart, Path, State};
use axum::http::{header, HeaderName, HeaderValue};
use axum::routing::{get, post};
use axum::{debug_handler, Json, Router};
use uuid::Uuid;

pub fn router(state: AppState) -> Router {
    let images_router = Router::new()
        .route("/images", post(create_image).get(get_images))
        .route(
            "/images/{image_id}",
            get(download_image).delete(delete_image),
        )
        .layer(DefaultBodyLimit::disable())
        .layer(DefaultBodyLimit::max(250 * 1024 * 1024 /* 250mb */))
        .with_state(state);

    Router::new().nest("/projects/{project_id}", images_router)
}

async fn create_image(
    Path(project_id): Path<Uuid>,
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<Vec<Image>>> {
    let mut result = vec![];
    while let Some(field) = multipart.next_field().await? {
        let file_name = field
            .file_name()
            .ok_or(AppError::MultipartMissing("filename"))?
            .to_string();
        let content_type = field
            .content_type()
            .ok_or(AppError::MultipartMissing("Content-Type"))?
            .to_string();

        if !content_type.starts_with("image/") {
            return Err(anyhow!("Content-Type is not an image").into());
        }

        let data = field.bytes().await?;

        let image = controller::create_image(project_id, file_name, data, &state).await?;
        result.push(image);
    }
    Ok(Json(result))
}

#[debug_handler]
async fn get_images(
    Path(project_id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<Vec<Image>>> {
    Ok(Json(controller::get_images(project_id, &state).await?))
}

#[debug_handler]
async fn download_image(
    Path((project_id, image_id)): Path<(Uuid, Uuid)>,
    State(state): State<AppState>,
) -> Result<([(HeaderName, HeaderValue); 2], Vec<u8>)> {
    let (file_name, image_bytes) = controller::get_image(project_id, image_id, &state).await?;

    let file_header = format!("attachment; filename={}", file_name);
    Ok((
        [
            (header::CONTENT_TYPE, HeaderValue::from_static("image/png")),
            (
                header::CONTENT_DISPOSITION,
                HeaderValue::from_str(&file_header).context("Invalid header value")?,
            ),
        ],
        image_bytes,
    ))
}

#[debug_handler]
async fn delete_image(
    Path((project_id, image_id)): Path<(Uuid, Uuid)>,
    State(state): State<AppState>,
) -> Result<Json<Image>> {
    let image = controller::delete_image(image_id, project_id, &state).await?;
    let image = image.ok_or(AppError::EntityNotFound)?;
    Ok(Json(image))
}
