use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::num::ParseIntError;

#[derive(Debug)]
pub enum AppError {
    Database(libsql::Error),
    NotFound,
    BadRequest,
    Internal(String),
}

impl From<libsql::Error> for AppError {
    fn from(err: libsql::Error) -> Self {
        AppError::Database(err)
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Internal(err.to_string())
    }
}

impl From<ParseIntError> for AppError {
    fn from(_: ParseIntError) -> Self {
        AppError::BadRequest
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::BadRequest => StatusCode::BAD_REQUEST,
            AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        status.into_response()
    }
}
