use crate::tools::Tool;
use serde::{de, Deserialize, Deserializer, Serialize};
use std::path::PathBuf;

#[derive(Deserialize, Serialize, Debug)]
pub struct ImageUriParams {
    #[serde(rename = "inputImageURI")]
    pub input_image_uri: PathBuf,
    #[serde(rename = "outputImageURI", skip_serializing_if = "Option::is_none")]
    pub output_image_uri: Option<PathBuf>,
}
#[derive(Debug)]
pub struct ToolParts {
    pub tool: Tool,
    pub image_uris: ImageUriParams,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RequestMessage {
    pub message_id: String,
    pub timestamp: String,
    #[serde(flatten)]
    pub params: ToolParts,
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
    use crate::message::RequestMessage;
    use std::path::PathBuf;
    use std::str::FromStr;

    #[test]
    fn test_deserialize() {
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
        assert_eq!(input.timestamp, "2024-11-01T12:00:00Z");
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
}
