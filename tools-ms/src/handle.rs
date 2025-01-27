use crate::handle::HandleRequestError::{
    ImageOpenError, ImageReadError, ImageSaveCreateFoldersError, ImageSaveError, ToolApplyError,
};
use crate::message::RequestMessage;
use crate::tools;
use std::path::PathBuf;
use thiserror::Error;
use tracing::{info, instrument};

#[derive(Debug)]
pub enum HandleRequestResult {
    Image(PathBuf),
    Text(String),
}

#[derive(Debug, Error)]
pub enum HandleRequestError {
    #[error("Failed to save image: {0}")]
    ImageSaveError(photon_rs::native::Error),
    #[error("Failed to create folders: {0}")]
    ImageSaveCreateFoldersError(tokio::io::Error),
    #[error("Failed to open image: {0}")]
    ImageOpenError(tokio::io::Error),
    #[error("Failed to read image: {0}")]
    ImageReadError(photon_rs::native::Error),
    #[error("Failed to apply tool: {0}")]
    ToolApplyError(anyhow::Error),
    #[error("Missing output path")]
    MissingOutputPath,
}

#[instrument(skip(request), fields(message_id = request.message_id))]
pub async fn handle_request(
    request: RequestMessage,
) -> Result<HandleRequestResult, HandleRequestError> {
    info!("Handling request");
    let path = request.params.image_uris.input_image_uri;
    let image_bytes = tokio::fs::read(&path.clone())
        .await
        .map_err(ImageOpenError)?;
    let image = photon_rs::native::open_image_from_bytes(&image_bytes).map_err(ImageReadError)?;
    info!(
        raw_pixels_len = image.get_raw_pixels().len(),
        "Loaded image"
    );
    let result = request.params.tool.apply(image).map_err(ToolApplyError)?;

    let result_name = match &result {
        tools::ToolApplyResult::Image(img) => format!("image({})", img.get_raw_pixels().len()),
        tools::ToolApplyResult::Text(text) => format!("text({})", text),
    };
    info!(result = result_name, "Applied tool to image");

    match result {
        tools::ToolApplyResult::Image(image) => {
            if let Some(path) = request.params.image_uris.output_image_uri {
                info!(path = path.to_string_lossy().as_ref(), "Saving image");

                if let Some(p) = path.parent() {
                    tokio::fs::create_dir_all(p)
                        .await
                        .map_err(ImageSaveCreateFoldersError)?;
                }

                photon_rs::native::save_image(image, &path).map_err(ImageSaveError)?;
                Ok(HandleRequestResult::Image(path))
            } else {
                Err(HandleRequestError::MissingOutputPath)
            }
        }
        tools::ToolApplyResult::Text(text) => Ok(HandleRequestResult::Text(text)),
    }
}
