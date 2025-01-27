use crate::error::AppError::Forbidden;
use crate::error::{AppError, Result};
use crate::image::controller;
use crate::image::model::Image;
use crate::user::AccessTokenClaims;
use crate::{project, AppState};
use axum::extract::{DefaultBodyLimit, Multipart, Path, State};
use axum::http::{header, HeaderValue};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{debug_handler, Json, Router};
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use uuid::Uuid;
use zip::ZipArchive;

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

#[debug_handler]
async fn create_image(
    Path(project_id): Path<Uuid>,
    user: AccessTokenClaims,
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse> {
    if !project::controller::can_modify(project_id, user.sub, &state).await? {
        return Err(Forbidden);
    }

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

        let data = field.bytes().await?;

        if content_type.starts_with("image/") {
            let image = controller::create_image(project_id, file_name, data, &state).await?;
            result.push(image);
        } else if content_type == "application/zip" || file_name.to_lowercase().ends_with(".zip") {
            let state_clone = state.clone();
            let project_id_clone = project_id;
            let zip_data = data.to_vec();

            let extracted_images =
                tokio::task::spawn_blocking(move || -> Result<Vec<(String, Vec<u8>)>> {
                    let cursor = Cursor::new(zip_data);
                    let mut zip = ZipArchive::new(cursor).map_err(|_| AppError::InvalidZip)?;

                    let mut images = Vec::new();

                    for i in 0..zip.len() {
                        let mut file = zip.by_index(i).map_err(|_| AppError::InvalidZip)?;

                        if file.is_dir() {
                            continue;
                        }

                        let extracted_file_name = file.name().to_string();

                        if is_image_file(&extracted_file_name) {
                            let mut buffer = Vec::with_capacity(file.size() as usize);
                            use std::io::Read;
                            file.read_to_end(&mut buffer)?;

                            images.push((extracted_file_name, buffer));
                        }
                    }

                    Ok(images)
                })
                .await
                .map_err(|_| AppError::InternalError)??;

            for (extracted_file_name, buffer) in extracted_images {
                let image = controller::create_image(
                    project_id_clone,
                    extracted_file_name,
                    buffer.into(),
                    &state_clone,
                )
                .await?;
                result.push(image);
            }
        } else {
            return Err(AppError::NotAnImage(content_type));
        }
    }
    Ok(Json(result))
}

fn is_image_file(file_name: &str) -> bool {
    let lower = file_name.to_lowercase();
    lower.ends_with(".png")
        || lower.ends_with(".jpg")
        || lower.ends_with(".jpeg")
        || lower.ends_with(".gif")
        || lower.ends_with(".bmp")
        || lower.ends_with(".tiff")
        || lower.ends_with(".webp")
}

#[derive(Debug, Serialize, Deserialize)]
struct ImageWithUrl {
    #[serde(flatten)]
    image: Image,
    url: String,
}

#[debug_handler]
async fn get_images(
    Path(project_id): Path<Uuid>,
    user: AccessTokenClaims,
    State(state): State<AppState>,
) -> Result<impl IntoResponse> {
    if !project::controller::can_modify(project_id, user.sub, &state).await? {
        return Err(Forbidden);
    }

    let images = controller::get_original_images(project_id, &state)
        .await?
        .into_iter()
        .map(|image| {
            let url = format!(
                "{}/api/v1/projects/{}/images/{}",
                state.config.picturas_public_url, project_id, image.id
            );
            ImageWithUrl { image, url }
        })
        .collect::<Vec<_>>();

    Ok(Json(images))
}

#[debug_handler]
async fn download_image(
    Path((project_id, image_id)): Path<(Uuid, Uuid)>,
    user: AccessTokenClaims,
    State(state): State<AppState>,
) -> Result<impl IntoResponse> {
    if !project::controller::can_modify(project_id, user.sub, &state).await? {
        return Err(Forbidden);
    }

    let image_bytes = controller::get_image(project_id, image_id, &state).await?;
    Ok((
        [(header::CONTENT_TYPE, HeaderValue::from_static("image/png"))],
        image_bytes,
    ))
}

#[debug_handler]
async fn delete_image(
    Path((project_id, image_id)): Path<(Uuid, Uuid)>,
    user: AccessTokenClaims,
    State(state): State<AppState>,
) -> Result<impl IntoResponse> {
    if !project::controller::can_modify(project_id, user.sub, &state).await? {
        return Err(Forbidden);
    }

    let image = controller::delete_image(image_id, project_id, &state).await?;
    let image = image.ok_or(AppError::EntityNotFound)?;
    Ok(Json(image))
}
