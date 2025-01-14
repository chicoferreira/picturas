use crate::tool::amqp::rabbit_controller::ToolQueue;
use crate::AppState;
use clap::Parser;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Parser)]
pub struct Config {
    #[arg(long, env)]
    pub pg_host: String,
    #[arg(long, env, default_value_t = 5432)]
    pub pg_port: u16,
    #[arg(long, env)]
    pub pg_user: String,
    #[arg(long, env)]
    pub pg_password: String,
    #[arg(long, env)]
    pub rabbitmq_host: String,
    #[arg(long, env, default_value_t = 5672)]
    pub rabbitmq_port: u16,
    #[arg(long, env)]
    pub rabbitmq_user: String,
    #[arg(long, env)]
    pub rabbitmq_password: String,
    #[arg(long, env)]
    pub rabbitmq_results_exchange: String,
    #[arg(long, env)]
    pub rabbitmq_results_routing_key: String,
    #[arg(long, env)]
    pub picturas_bind_address: String,
    #[arg(long, env)]
    pub picturas_image_folder: PathBuf,
    #[arg(long, env)]
    pub picturas_public_url: String,
    #[arg(long, env, use_value_delimiter = true, value_parser = parse_tool_queue)]
    pub picturas_available_tools: Vec<ToolQueue>,
}

fn parse_tool_queue(src: &str) -> Result<ToolQueue, String> {
    let (name, routing_key) = src.split_once(':').unwrap_or((src, src));
    Ok(ToolQueue {
        name: name.to_string(),
        routing_key: routing_key.to_string(),
    })
}

pub fn generate_image_version_folder_uri(project_uuid: Uuid, state: &AppState) -> PathBuf {
    state
        .config
        .picturas_image_folder
        .join(project_uuid.to_string())
        .join("output")
}

pub fn generate_image_version_output_uri(
    project_uuid: Uuid,
    original_image_uuid: Uuid,
    new_image_uuid: Uuid,
    state: &AppState,
) -> PathBuf {
    generate_image_version_folder_uri(project_uuid, state)
        .join(original_image_uuid.to_string())
        .join(new_image_uuid.to_string())
        .with_extension("png")
}

pub fn generate_image_uri(
    project_uuid: Uuid,
    image_uuid: Uuid,
    image_name: &str,
    state: &AppState,
) -> PathBuf {
    let buf = PathBuf::from(image_name);
    let extension_from_name = buf.extension().unwrap_or_default();
    state
        .config
        .picturas_image_folder
        .join(project_uuid.to_string())
        .join(image_uuid.to_string())
        .with_extension(extension_from_name)
}
