use axum::response::IntoResponse;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("database error: {0}")]
    SqlxError(#[from] sqlx::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        log::error!("{}", self);
        let body = self.to_string();
        let status = match self {
            Error::SqlxError(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, body).into_response()
    }
}
