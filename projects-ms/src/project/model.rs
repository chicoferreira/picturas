use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// The project model.
#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    /// The unique identifier of the project.
    pub id: Uuid,
    /// The name of the project.
    pub name: String,
    /// The user id of the project.
    pub user_id: Uuid,
    /// The date and time the project was created.
    pub created_at: DateTime<Utc>,
    /// The date and time the project was last updated.
    pub updated_at: DateTime<Utc>,
}
