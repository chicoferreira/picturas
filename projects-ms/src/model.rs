use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// The image model.
#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    /// The unique identifier of the image.
    pub id: Uuid,
    /// The project associated with the image.
    pub project_id: Uuid,
    /// The URI of the image file.
    pub uri: String,
}

/// The tool model.
#[derive(Debug, Serialize, Deserialize)]
pub struct Tool {
    /// The unique identifier of the tool.
    pub id: Uuid,
    /// The index of the tool in the project (0 should be the first tool to be applied).
    pub position: u16,
    /// The procedure to be applied to the image.
    pub procedure: String,
    /// The parameters of the procedure.
    pub parameters: Vec<(String, String)>,
    /// The project associated with the tool.
    pub project_id: Uuid,
}
