use crate::AppState;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

/// The image model.
#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    /// The unique identifier of the image.
    pub id: Uuid,
    /// The image name.
    pub name: String,
    /// The project associated with the image.
    pub project_id: Uuid,
}

impl Image {
    pub fn get_uri(&self, state: &AppState) -> PathBuf {
        let buf = PathBuf::from(&self.name);
        let extension_from_name = buf.extension().unwrap_or_default();
        state
            .config
            .picturas_image_folder
            .join(self.project_id.to_string())
            .join(self.id.to_string())
            .with_extension(extension_from_name)
    }
}
