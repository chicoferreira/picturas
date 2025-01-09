use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use thiserror::Error;
use uuid::Uuid;

// This user will be extracted from the request headers.
pub struct User {
    pub uuid: Uuid,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Error)]
pub enum UserExtractionError {
    #[error("Missing header: {0}")]
    MissingHeader(&'static str),

    #[error("Invalid UUID in header: {0}")]
    InvalidUuid(&'static str),
}

// TODO: change this to a more structured error response.
impl IntoResponse for UserExtractionError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::BAD_REQUEST, self.to_string()).into_response()
    }
}

impl<S: Sync + Send> FromRequestParts<S> for User {
    type Rejection = UserExtractionError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        const HEADER_ID: &str = "x-user-id";
        const HEADER_NAME: &str = "x-user-name";
        const HEADER_EMAIL: &str = "x-user-email";

        let get_header = |key: &'static str| -> Result<&str, UserExtractionError> {
            parts
                .headers
                .get(key)
                .and_then(|value| value.to_str().ok())
                .ok_or(UserExtractionError::MissingHeader(key))
        };

        let id_str = get_header(HEADER_ID)?;
        let id =
            Uuid::parse_str(id_str).map_err(|_| UserExtractionError::InvalidUuid(HEADER_ID))?;
        let name = get_header(HEADER_NAME)?.to_string();
        let email = get_header(HEADER_EMAIL)?.to_string();

        Ok(User {
            uuid: id,
            name,
            email,
        })
    }
}
