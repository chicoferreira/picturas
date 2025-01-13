use crate::message::RequestMessage;
use crate::tools;
use snafu::prelude::*;
use std::path::PathBuf;
use tracing::{info, span};

#[derive(Debug)]
pub enum HandleRequestResult {
    Image(PathBuf),
    Text(String),
}

#[derive(Debug, Snafu)]
pub enum HandleRequestError {
    #[snafu(display("Failed to save image"))]
    ImageSaveError {
        source: photon_rs::native::Error,
        path: PathBuf,
    },
    #[snafu(display("Failed to create folders"))]
    ImageSaveCreateFoldersError { source: tokio::io::Error },
    #[snafu(display("Failed to open image"))]
    ImageOpenError {
        source: photon_rs::native::Error,
        path: PathBuf,
    },
    #[snafu(display("Failed to apply tool"))]
    ToolApplyError { source: anyhow::Error },
    #[snafu(display("Missing output path"))]
    MissingOutputPath,
}

pub async fn handle_request(
    request: RequestMessage,
) -> Result<HandleRequestResult, HandleRequestError> {
    let span = span!(tracing::Level::INFO, "handle_request", message_id = %request.message_id);
    let _enter = span.enter();

    info!("Handling request");
    let path = request.params.image_uris.input_image_uri;
    let image = photon_rs::native::open_image(&path).context(ImageOpenSnafu { path })?;
    info!(
        raw_pixels_len = image.get_raw_pixels().len(),
        "Loaded image"
    );
    let result = request.params.tool.apply(image).context(ToolApplySnafu)?;

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
                        .context(ImageSaveCreateFoldersSnafu)?;
                }

                photon_rs::native::save_image(image, &path)
                    .context(ImageSaveSnafu { path: path.clone() })?;
                Ok(HandleRequestResult::Image(path))
            } else {
                Err(HandleRequestError::MissingOutputPath)
            }
        }
        tools::ToolApplyResult::Text(text) => Ok(HandleRequestResult::Text(text)),
    }
}
