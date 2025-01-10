use axum::extract::multipart::MultipartError;
use axum::response::IntoResponse;
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
    #[error("{0}")]
    Other(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        log::error!("{}", self);
        let body = self.to_string();
        let status = match self {
            AppError::Sqlx(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            AppError::EntityNotFound => axum::http::StatusCode::NOT_FOUND,
            AppError::Multipart(_) => axum::http::StatusCode::BAD_REQUEST,
            AppError::MultipartMissing(_) => axum::http::StatusCode::BAD_REQUEST,
            AppError::Io(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Other(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, body).into_response()
    }
}
