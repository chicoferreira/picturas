use crate::AppState;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::JsonValue;
use std::path::PathBuf;
use uuid::Uuid;

/// The tool model.
#[derive(Debug, Serialize, Deserialize)]
pub struct Tool {
    /// The unique identifier of the tool.
    pub id: Uuid,
    /// The project associated with the tool.
    pub project_id: Uuid,
    /// The index of the tool in the project (0 should be the first tool to be applied).
    pub position: i32,
    /// The procedure to be applied to the image.
    pub procedure: String,
    /// The parameters of the procedure.
    pub parameters: JsonValue,
}

/// An image with a tool applied to it.
#[derive(Debug, Serialize, Deserialize)]
pub struct ImageVersion {
    /// The unique identifier of the image version.
    pub id: Uuid,
    /// The image associated with the image version.
    pub image_id: Uuid,
    /// The tool that created this image version.
    pub tool_id: Uuid,
    /// The text result of the tool if any (e.g. OCR result).
    pub text_result: Option<String>,
    /// The timestamp of the image version creation.
    pub created_at: DateTime<Utc>,
}

impl ImageVersion {
    pub fn get_uri(&self, state: &AppState) -> PathBuf {
        state
            .config
            .picturas_image_folder
            .join(self.image_id.to_string())
            .join(self.id.to_string())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RequestedTool {
    pub procedure: String,
    pub parameters: Vec<(String, serde_json::Value)>,
}
