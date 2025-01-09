use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// The project model.
#[derive(Debug, Serialize, Deserialize)]
struct Project {
    /// The unique identifier of the project.
    id: Uuid,
    /// The name of the project.
    name: String,
    /// The description of the project.
    user_id: Uuid,
}

/// The image model.
#[derive(Debug, Serialize, Deserialize)]
struct Image {
    /// The unique identifier of the image.
    id: Uuid,
    /// The project associated with the image.
    project_id: Uuid,
    /// The URI of the image file.
    uri: String,
}

/// The tool model.
#[derive(Debug, Serialize, Deserialize)]
struct Tool {
    /// The unique identifier of the tool.
    id: Uuid,
    /// The index of the tool in the project (0 should be the first tool to be applied).
    position: u16,
    /// The procedure to be applied to the image.
    procedure: String,
    /// The parameters of the procedure.
    parameters: Vec<(String, String)>,
    /// The project associated with the tool.
    project_id: Uuid,
}
