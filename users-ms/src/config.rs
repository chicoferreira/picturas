use clap::Parser;
use jsonwebtoken::{DecodingKey, EncodingKey};
use std::time::Duration;

#[derive(Parser)]
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
    pub pg_database: String,
    #[arg(long, env)]
    pub redis_host: String,
    #[arg(long, env, default_value_t = 6379)]
    pub redis_port: u16,
    #[arg(long, env)]
    pub redis_password: Option<String>,
    #[arg(long, env)]
    pub bind_ip: String,
    #[arg(long, env, default_value_t = 8080)]
    pub bind_port: u16,
    #[arg(long, env, value_parser = parse_duration)]
    pub access_token_max_age: Duration,
    #[arg(long, env, value_parser = parse_duration)]
    pub refresh_token_max_age: Duration,
    #[arg(long, env, value_parser = load_decoding_key_from_file)]
    pub access_token_public_key: DecodingKey,
    #[arg(long, env, value_parser = load_encoding_key_from_file)]
    pub access_token_private_key: EncodingKey,
    #[arg(long, env, value_parser = load_decoding_key_from_file)]
    pub refresh_token_public_key: DecodingKey,
    #[arg(long, env, value_parser = load_encoding_key_from_file)]
    pub refresh_token_private_key: EncodingKey,
}

fn parse_duration(s: &str) -> Result<Duration, std::num::ParseIntError> {
    Ok(Duration::from_secs(60 * s.parse::<u64>()?))
}

fn load_decoding_key_from_file(path: &str) -> Result<DecodingKey, std::io::Error> {
    Ok(DecodingKey::from_rsa_pem(&std::fs::read(path)?).unwrap())
}

fn load_encoding_key_from_file(path: &str) -> Result<EncodingKey, std::io::Error> {
    Ok(EncodingKey::from_rsa_pem(&std::fs::read(path)?).unwrap())
}
