use crate::message::RequestMessage;

mod message;
mod tools;

async fn handle_request(request: RequestMessage) -> anyhow::Result<()> {
    let image = photon_rs::native::open_image(request.params.image_uris.input_image_uri)
        .expect("Failed to open image");

    let result = request
        .params
        .tool
        .apply(image)
        .expect("Failed to apply tool");

    match result {
        tools::ToolApplyResult::Image(image) => {
            if let Some(output_image_uri) = request.params.image_uris.output_image_uri {
                photon_rs::native::save_image(image, output_image_uri)
                    .expect("Failed to save image");
            } else {
                // TODO: log ignoring output
            }
        }
        tools::ToolApplyResult::Text(text) => {
            println!("{}", text);
        }
    }

    // TODO: send reponse

    Ok(())
}

#[tokio::main]
async fn main() {
    let input = r#"{
        "messageId": "request-2",
        "timestamp": "2024-11-01T12:00:00Z",
        "procedure": "rotate",
        "parameters": {
            "inputImageURI": "./images/request-1.png",
            "outputImageURI": "./images/request-2-out.jpg",
            "angle": -90
        }
    }"#;

    let input: RequestMessage = serde_json::from_str(input).expect("Failed to parse input");

    handle_request(input)
        .await
        .expect("Failed to handle request");
}
