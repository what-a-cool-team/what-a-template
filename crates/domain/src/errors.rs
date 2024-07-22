use std::collections::HashMap;

use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse {
    pub errors: HashMap<String, Vec<String>>,
}

impl ErrorResponse {
    pub fn new(error: String) -> Self {
        let mut error_map: HashMap<String, Vec<String>> = HashMap::new();
        error_map.insert("message".to_owned(), vec![error]);
        Self { errors: error_map }
    }
}

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Invalid request")]
    BadRequest,

    #[error("Unauthorized access to the resource")]
    Unauthorized,

    #[error("Unprivileged access")]
    Forbidden,

    #[error("{0}")]
    NotFound(String),

    #[error("{0}")]
    Conflict(String),

    #[error("Unexpected error has occurred")]
    InternalServerError(String),

    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Self::BadRequest => (StatusCode::BAD_REQUEST, Self::BadRequest.to_string()),
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, Self::Unauthorized.to_string()),
            Self::Forbidden => (StatusCode::FORBIDDEN, Self::Forbidden.to_string()),
            Self::NotFound(err) => (StatusCode::NOT_FOUND, err),
            Self::Conflict(err) => (StatusCode::CONFLICT, err),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Unexpected error has occurred"),
            ),
        };

        let body = Json(ErrorResponse::new(error_message));

        (status, body).into_response()
    }
}
