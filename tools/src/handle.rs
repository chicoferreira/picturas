use crate::message::RequestMessage;
use crate::tools;
use snafu::prelude::*;
use std::path::PathBuf;

pub enum HandleRequestResult {
    Image(PathBuf),
    Text(String),
}

#[derive(Debug, Snafu)]
pub enum HandleRequestError {
    #[snafu(display("Failed to open image"))]
    ImageSaveError {
        source: photon_rs::native::Error,
        path: PathBuf,
    },
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
    let path = request.params.image_uris.input_image_uri;
    let image = photon_rs::native::open_image(&path).context(ImageOpenSnafu { path })?;
    let result = request.params.tool.apply(image).context(ToolApplySnafu)?;

    match result {
        tools::ToolApplyResult::Image(image) => {
            if let Some(path) = request.params.image_uris.output_image_uri {
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
