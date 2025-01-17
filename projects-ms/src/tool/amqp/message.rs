use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestMessage {
    pub message_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub procedure: String,
    pub parameters: HashMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseMessage {
    pub message_id: String,
    pub correlation_id: Uuid,
    pub timestamp: DateTime<Utc>,
    #[serde(flatten)]
    pub status: ResponseStatus,
    pub metadata: Metadata,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum ResponseStatus {
    #[serde(rename = "success")]
    Success { output: OutputObject },
    #[serde(rename = "error")]
    Error { error: ErrorObject },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputObject {
    #[serde(rename = "type")]
    pub kind: OutputType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputType {
    Image,
    Text,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorObject {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub processing_time: f64,
    pub microservice: String,
}
