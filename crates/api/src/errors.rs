use std::collections::HashMap;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use thiserror::Error;
use serde::{Deserialize, Serialize};

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
    BadRequest(String),

    #[error("Unauthorized access to the resource")]
    Unauthorized,

    #[error("Unprivileged access")]
    Forbidden,

    #[error("Unexpected error has occurred")]
    InternalServerError,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Self::BadRequest(err) => (StatusCode::BAD_REQUEST, err),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Unexpected error has occurred"),
            )
        };

        let body = Json(ErrorResponse::new(error_message));

        (status, body).into_response()
    }
}
