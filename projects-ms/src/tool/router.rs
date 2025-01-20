use crate::error::AppError::Forbidden;
use crate::error::Result;
use crate::project::controller;
use crate::tool::controller::ImageVersionWithUrl;
use crate::tool::model::RequestedTool;
use crate::tool::websocket;
use crate::user::AccessTokenClaims;
use crate::{image, tool, AppState};
use axum::body::Bytes;
use axum::extract::{Path, State};
use axum::http::{header, HeaderMap, HeaderValue};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{debug_handler, Json, Router};
use serde_json::json;
use uuid::Uuid;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route(
            "/projects/{project_id}/tools",
            get(get_tools).post(add_tool).put(put_tools),
        )
        .route("/projects/{project_id}/tools/apply", post(apply_tools))
        .route(
            "/projects/{project_id}/tools/images",
            get(get_image_versions),
        )
        .route(
            "/projects/{project_id}/tools/images/{image_version_id}",
            get(download_image_version),
        )
        .route(
            "/projects/{project_id}/tools/imageszip",
            get(download_image_versions_zip),
        )
        .with_state(state.clone())
        .merge(websocket::router(state))
}

#[debug_handler]
async fn get_tools(
    Path(project_id): Path<Uuid>,
    user: AccessTokenClaims,
    State(state): State<AppState>,
) -> Result<impl IntoResponse> {
    if !controller::can_modify(project_id, user.sub, &state).await? {
        return Err(Forbidden);
    }

    Ok(tool::controller::get_applied_tools(project_id, &state)
        .await
        .map(Json))
}

#[debug_handler]
async fn add_tool(
    Path(project_id): Path<Uuid>,
    user: AccessTokenClaims,
    State(state): State<AppState>,
    Json(tool): Json<RequestedTool>,
) -> Result<impl IntoResponse> {
    if !controller::can_modify(project_id, user.sub, &state).await? {
        return Err(Forbidden);
    }

    Ok(tool::controller::add_tool(project_id, tool, &state)
        .await
        .map(Json))
}

#[debug_handler]
async fn put_tools(
    Path(project_id): Path<Uuid>,
    user: AccessTokenClaims,
    State(state): State<AppState>,
    Json(tools): Json<Vec<RequestedTool>>,
) -> Result<impl IntoResponse> {
    if !controller::can_modify(project_id, user.sub, &state).await? {
        return Err(Forbidden);
    }

    Ok(tool::controller::update_tools(project_id, tools, &state)
        .await
        .map(Json))
}

#[derive(serde::Deserialize)]
struct ApplyToolsRequest {
    filter_images: Option<Vec<Uuid>>,
}

#[debug_handler]
async fn apply_tools(
    Path(project_id): Path<Uuid>,
    user: AccessTokenClaims,
    State(state): State<AppState>,
    Json(image_ids): Json<ApplyToolsRequest>,
) -> Result<impl IntoResponse> {
    if !controller::can_modify(project_id, user.sub, &state).await? {
        return Err(Forbidden);
    }

    let mut images = image::controller::get_original_images(project_id, &state).await?;

    if let Some(filter_images) = &image_ids.filter_images {
        images.retain(|image| filter_images.contains(&image.id));
    }

    tool::controller::apply_added_tools(project_id, user.sub, &images, &state).await?;

    let image_ids = images.iter().map(|image| image.id).collect::<Vec<_>>();

    Ok(Json(json!({
        "image_ids": image_ids,
        "message": "Hook to websocket to get realtime results",
    })))
}

#[debug_handler]
async fn get_image_versions(
    Path(project_id): Path<Uuid>,
    user: AccessTokenClaims,
    State(state): State<AppState>,
) -> Result<impl IntoResponse> {
    if !controller::can_modify(project_id, user.sub, &state).await? {
        return Err(Forbidden);
    }

    let images: Vec<_> = tool::controller::get_image_versions(project_id, &state)
        .await?
        .into_iter()
        .map(|image_version| ImageVersionWithUrl::from_image_version(image_version, &state))
        .collect();

    Ok(Json(images))
}

#[debug_handler]
async fn download_image_version(
    Path((project_id, image_version_id)): Path<(Uuid, Uuid)>,
    user: AccessTokenClaims,
    State(state): State<AppState>,
) -> Result<impl IntoResponse> {
    if !controller::can_modify(project_id, user.sub, &state).await? {
        return Err(Forbidden);
    }

    let image_bytes =
        tool::controller::load_image_version(project_id, image_version_id, &state).await?;

    Ok((
        [(header::CONTENT_TYPE, HeaderValue::from_static("image/png"))],
        image_bytes,
    ))
}

#[derive(serde::Deserialize)]
struct DownloadImageVersionsZipRequest {
    tool_id: Uuid,
}

#[debug_handler]
async fn download_image_versions_zip(
    Path(project_id): Path<Uuid>,
    user: AccessTokenClaims,
    State(state): State<AppState>,
    Json(body): Json<DownloadImageVersionsZipRequest>,
) -> Result<impl IntoResponse> {
    if !controller::can_modify(project_id, user.sub, &state).await? {
        return Err(Forbidden);
    }

    let zip_bytes =
        tool::controller::load_image_versions_zip(project_id, body.tool_id, &state).await?;

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/zip"),
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        HeaderValue::from_static("attachment; filename=\"images.zip\""),
    );

    Ok((headers, Bytes::from(zip_bytes)))
}
