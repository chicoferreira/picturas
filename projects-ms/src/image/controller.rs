use crate::error::Result;
use crate::image::model::Image;
use crate::AppState;
use axum::body::Bytes;
use tokio::fs::File;
use tokio::io::{AsyncWriteExt, BufWriter};
use tracing::info;
use uuid::Uuid;

pub async fn create_image(
    project_uuid: Uuid,
    image_name: String,
    image_bytes: Bytes,
    state: &AppState,
) -> Result<Image> {
    info!("Creating image with name: {}", image_name);
    let uuid = Uuid::new_v4();

    let image = Image {
        id: uuid,
        name: image_name,
        project_id: project_uuid,
    };

    let path = image.get_uri(state);
    tokio::fs::create_dir_all(&path.parent().unwrap()).await?;
    let file = File::create(&path).await?;
    let mut writer = BufWriter::new(file);
    writer.write_all(&image_bytes).await?;

    sqlx::query!(
        "INSERT INTO images (id, name, project_id) VALUES ($1, $2, $3)",
        image.id,
        image.name,
        image.project_id
    )
    .execute(&state.db_pool)
    .await?;

    info!(
        id = ?image.id,
        path = ?path,
        "Image created"
    );
    Ok(image)
}

pub async fn get_original_images(project_uuid: Uuid, state: &AppState) -> Result<Vec<Image>> {
    info!("Fetching original images for project ID: {}", project_uuid);
    let images = sqlx::query_as!(
        Image,
        "SELECT id, name, project_id FROM images WHERE project_id = $1",
        project_uuid
    )
    .fetch_all(&state.db_pool)
    .await?;

    info!("Fetched {} images", images.len());
    Ok(images)
}

pub async fn delete_image(
    image_uuid: Uuid,
    project_uuid: Uuid,
    state: &AppState,
) -> Result<Option<Image>> {
    info!("Deleting image with ID: {}", image_uuid);
    let image = sqlx::query_as!(
        Image,
        "SELECT id, name, project_id FROM images WHERE id = $1 AND project_id = $2",
        image_uuid,
        project_uuid
    )
    .fetch_optional(&state.db_pool)
    .await?;

    let Some(image) = image else {
        return Ok(None);
    };

    sqlx::query!("DELETE FROM images WHERE id = $1", image.id)
        .execute(&state.db_pool)
        .await?;

    tokio::fs::remove_file(image.get_uri(state)).await?;

    info!(
        id = ?image.id,
        "Deleted image"
    );
    Ok(Some(image))
}

pub async fn get_image(project_id: Uuid, image_id: Uuid, state: &AppState) -> Result<Vec<u8>> {
    info!("Fetching image with ID: {}", image_id);
    let image = sqlx::query_as!(
        Image,
        "SELECT id, name, project_id FROM images WHERE id = $1 AND project_id = $2",
        image_id,
        project_id
    )
    .fetch_one(&state.db_pool)
    .await?;

    let path = image.get_uri(state);
    let file = tokio::fs::read(&path).await?;

    info!(
        id = ?image.id,
        path = ?path,
        "Fetched image"
    );
    Ok(file)
}
