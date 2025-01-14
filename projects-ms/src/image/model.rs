use crate::{config, AppState};
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
        config::generate_image_uri(self.project_id, self.id, &self.name, state)
    }
}
