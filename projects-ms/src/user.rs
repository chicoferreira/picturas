use crate::error::AppError;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use uuid::Uuid;

// This user will be extracted from the request headers.
#[derive(Debug)]
pub struct User {
    pub uuid: Uuid,
    // pub name: String,
    // pub email: String,
}

impl<S: Sync + Send> FromRequestParts<S> for User {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        const HEADER_ID: &str = "x-user-id";
        // const HEADER_NAME: &str = "x-user-name";
        // const HEADER_EMAIL: &str = "x-user-email";

        let get_header = |key: &'static str| -> Result<&str, AppError> {
            parts
                .headers
                .get(key)
                .and_then(|value| value.to_str().ok())
                .ok_or(AppError::MissingHeader(key))
        };

        let id_str = get_header(HEADER_ID)?;
        let id = Uuid::parse_str(id_str).map_err(|_| AppError::InvalidUuid(id_str.to_string()))?;
        // let name = get_header(HEADER_NAME)?.to_string();
        // let email = get_header(HEADER_EMAIL)?.to_string();

        Ok(User {
            uuid: id,
            // name,
            // email,
        })
    }
}
