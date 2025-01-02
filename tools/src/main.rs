use crate::tools::{Tool, ToolApplyResult};
use serde::{Deserialize, Serialize};

mod tools;
mod utils;

#[derive(Deserialize, Serialize)]
struct Input {
    image: String,
    tool: Tool,
}

fn main() {
    let input = r#"{
        "image": "...",
        "tool": {
            "add_border": { "size": 5, "color": [255, 0, 0] }
        }
    }"#;

    let input: Input = serde_json::from_str(input).expect("Failed to parse input");
    let img = utils::load_from_base64(&input.image).expect("Failed to parse image");

    match input.tool.apply(img).expect("Failed to apply tool") {
        ToolApplyResult::Image(image) => {
            let base64 = image.get_base64();
            println!("Image: {base64}");
        }
        ToolApplyResult::Text(text) => {
            println!("Text: \"{text}\"");
        }
    }
}
