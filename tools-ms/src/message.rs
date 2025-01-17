use crate::tools::Tool;
use chrono::{DateTime, Utc};
use serde::ser::SerializeMap;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RequestMessage {
    pub message_id: String,
    // pub timestamp: DateTime<Utc>,
    #[serde(flatten)]
    pub params: ToolParts,
}

#[derive(Debug)]
pub struct ToolParts {
    pub tool: Tool,
    pub image_uris: ImageUriParams,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ImageUriParams {
    #[serde(rename = "inputImageURI")]
    pub input_image_uri: PathBuf,
    #[serde(rename = "outputImageURI", skip_serializing_if = "Option::is_none")]
    pub output_image_uri: Option<PathBuf>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResponseMessage {
    pub message_id: String,
    pub correlation_id: String,
    pub timestamp: DateTime<Utc>,
    #[serde(flatten)]
    pub status: ResponseMessageStatus,
    pub metadata: Metadata,
}

#[derive(Debug)]
pub enum ResponseMessageStatus {
    Success(OutputType),
    Error(ErrorDetails),
}

#[derive(Serialize, Debug)]
pub struct ErrorDetails {
    pub code: String,
    pub message: String,
}

impl Serialize for ResponseMessageStatus {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Success(output_type) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("status", "success")?;
                map.serialize_entry("output", output_type)?;
                map.end()
            }
            Self::Error(error_details) => {
                let mut map = serializer.serialize_map(Some(4))?;
                map.serialize_entry("status", "error")?;
                map.serialize_entry("error", error_details)?;
                map.end()
            }
        }
    }
}
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum OutputType {
    Image {
        #[serde(rename = "imageURI")]
        image_uri: PathBuf,
    },
    Text {
        text: String,
    },
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub processing_time: f64,
    pub microservice: String,
}

// Custom deserialize is needed because serde_json can't merge two "parameters" objects into one
impl<'de> Deserialize<'de> for ToolParts {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;

        let procedure = value
            .get("procedure")
            .and_then(|v| v.as_str())
            .ok_or_else(|| de::Error::missing_field("procedure"))?
            .to_string();

        let parameters_obj = value
            .get("parameters")
            .and_then(|v| v.as_object())
            .ok_or_else(|| de::Error::missing_field("parameters"))?;

        let mut image_map = serde_json::Map::new();
        let mut tool_map = serde_json::Map::new();

        for (k, v) in parameters_obj {
            match k.as_str() {
                "inputImageURI" | "outputImageURI" => {
                    image_map.insert(k.clone(), v.clone());
                }
                _ => {
                    tool_map.insert(k.clone(), v.clone());
                }
            }
        }

        let image_url_value = serde_json::Value::Object(image_map);
        let image_uris: ImageUriParams =
            serde_json::from_value(image_url_value).map_err(de::Error::custom)?;

        let tool_obj = serde_json::json!({
            "procedure": procedure,
            "parameters": serde_json::Value::Object(tool_map)
        });

        let tool: Tool = serde_json::from_value(tool_obj).map_err(de::Error::custom)?;

        Ok(ToolParts { tool, image_uris })
    }
}

#[cfg(test)]
mod tests {
    use crate::message::{ErrorDetails, OutputType, RequestMessage, ResponseMessageStatus};
    use std::path::PathBuf;
    use std::str::FromStr;
    // use chrono::{DateTime, Utc};

    #[test]
    fn test_deserialize_request_message() {
        let input = r#"{
            "messageId": "request-2",
            "timestamp": "2024-11-01T12:00:00Z",
            "procedure": "rotate",
            "parameters": {
                "inputImageURI": "images/request-1-out.jpg",
                "outputImageURI": "images/request-2-out.jpg",
                "angle": -90
            }
        }"#;

        let input: RequestMessage = serde_json::from_str(input).expect("Failed to parse input");

        assert_eq!(input.message_id, "request-2");
        // assert_eq!(input.timestamp, "2024-11-01T12:00:00Z".parse::<DateTime<Utc>>().unwrap());
        assert_eq!(
            input.params.image_uris.input_image_uri,
            PathBuf::from_str("images/request-1-out.jpg").unwrap()
        );
        assert_eq!(
            input.params.image_uris.output_image_uri,
            Some("images/request-2-out.jpg".into())
        );
        assert_eq!(
            input.params.tool,
            crate::tools::Tool::Rotate { angle: -90.0 }
        );
    }

    #[test]
    fn test_serialize_response_message() {
        let response = crate::message::ResponseMessage {
            message_id: "completion-2".into(),
            correlation_id: "request-2".into(),
            timestamp: "2024-11-01T12:00:01Z".parse().unwrap(),
            status: ResponseMessageStatus::Success(OutputType::Image {
                image_uri: "images/request-2-out.jpg".into(),
            }),
            metadata: crate::message::Metadata {
                processing_time: 0.2f64,
                microservice: "picturas-rotate-tool-ms".into(),
            },
        };

        let output = serde_json::to_string(&response).expect("Failed to serialize response");
        let expected = r#"{"messageId":"completion-2","correlationId":"request-2","timestamp":"2024-11-01T12:00:01Z","status":"success","output":{"type":"image","imageURI":"images/request-2-out.jpg"},"metadata":{"processingTime":0.2,"microservice":"picturas-rotate-tool-ms"}}"#;
        assert_eq!(output, expected);

        let response = crate::message::ResponseMessage {
            message_id: "completion-20".into(),
            correlation_id: "request-20".into(),
            timestamp: "2024-11-01T12:01:00Z".parse().unwrap(),
            status: ResponseMessageStatus::Error(ErrorDetails {
                code: "INVALID_INPUT".into(),
                message: "The input file format is not supported.".into(),
            }),
            metadata: crate::message::Metadata {
                processing_time: 0.1f64,
                microservice: "picturas-crop-tool-ms".into(),
            },
        };

        let output = serde_json::to_string(&response).expect("Failed to serialize response");
        let expected = r#"{"messageId":"completion-20","correlationId":"request-20","timestamp":"2024-11-01T12:01:00Z","status":"error","error":{"code":"INVALID_INPUT","message":"The input file format is not supported."},"metadata":{"processingTime":0.1,"microservice":"picturas-crop-tool-ms"}}"#;
        assert_eq!(output, expected);
    }
}
