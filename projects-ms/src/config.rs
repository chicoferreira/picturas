use crate::tool::amqp::rabbit_controller::ToolQueue;
use clap::Parser;
use std::path::PathBuf;

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
