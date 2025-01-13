use crate::AppState;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::JsonValue;
use std::collections::HashMap;
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
    pub original_image_id: Uuid,
    /// The project associated with the image version.
    pub project_id: Uuid,
    /// The tool that created this image version.
    pub tool_id: Uuid,
    /// The text result of the tool if any (e.g. OCR result).
    pub text_result: Option<String>,
    /// The timestamp of the image version creation.
    pub created_at: DateTime<Utc>,
}

impl ImageVersion {
    pub fn get_uri(&self, state: &AppState) -> PathBuf {
        Self::generate_output_uri(self.project_id, self.original_image_id, self.id, state)
    }

    pub fn generate_output_uri(
        project_uuid: Uuid,
        original_image_uuid: Uuid,
        new_image_uuid: Uuid,
        state: &AppState,
    ) -> PathBuf {
        state
            .config
            .picturas_image_folder
            .join(project_uuid.to_string())
            .join("output")
            .join(original_image_uuid.to_string())
            .join(new_image_uuid.to_string())
            .with_extension("png")
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RequestedTool {
    pub procedure: String,
    pub parameters: HashMap<String, serde_json::Value>,
}

impl TryInto<RequestedTool> for Tool {
    type Error = serde_json::Error;

    fn try_into(self) -> Result<RequestedTool, Self::Error> {
        Ok(RequestedTool {
            procedure: self.procedure,
            parameters: serde_json::from_value(self.parameters)?,
        })
    }
}
