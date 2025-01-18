use crate::tool::amqp::rabbit_controller::RabbitMqControllerError;
use axum::extract::multipart::MultipartError;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("database error: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("entity not found")]
    EntityNotFound,
    #[error("multipart error: {0}")]
    Multipart(#[from] MultipartError),
    #[error("missing multipart field: {0}")]
    MultipartMissing(&'static str),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("rabbitmq controller error: {0}")]
    RabbitMq(#[from] RabbitMqControllerError),
    #[error("not an image: {0}")]
    NotAnImage(String),
    #[error("unauthorized")]
    Unauthorized,
    #[error(transparent)]
    JwtError(#[from] jsonwebtoken::errors::Error),
}

#[derive(Debug, serde::Serialize)]
struct ErrorBody {
    error: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        tracing::error!("{:?}", self);
        let status = match self {
            AppError::Sqlx(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Io(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::RabbitMq(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::EntityNotFound => StatusCode::NOT_FOUND,
            AppError::Multipart(_) => StatusCode::BAD_REQUEST,
            AppError::MultipartMissing(_) => StatusCode::BAD_REQUEST,
            AppError::NotAnImage(_) => StatusCode::BAD_REQUEST,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::JwtError(_) => StatusCode::UNAUTHORIZED,
        };

        let error = match self {
            AppError::Sqlx(_) => "Database error".to_string(),
            AppError::EntityNotFound => "Entity not found".to_string(),
            AppError::Multipart(err) => format!("Multipart error: {err}"),
            AppError::MultipartMissing(missing) => format!("Missing multipart field: {missing}"),
            AppError::Io(_) => "Internal IO error".to_string(),
            AppError::RabbitMq(_) => "Internal controller error".to_string(),
            AppError::NotAnImage(content_type) => format!("Not an image: {content_type}"),
            AppError::Unauthorized => "Unauthorized".to_string(),
            AppError::JwtError(_) => "Invalid token".to_string(),
        };

        let body = ErrorBody { error };

        (status, Json(body)).into_response()
    }
}
