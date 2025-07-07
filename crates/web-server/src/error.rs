use std::fmt;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use db::{PoolError, TokioPostgresError};

#[derive(Debug)]
pub enum CustomError {
    FaultySetup(String),
    Database(String),
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomError::FaultySetup(msg) => write!(f, "Faulty setup: {}", msg),
            CustomError::Database(msg) => write!(f, "Database error: {}", msg),
        }
    }
}

impl IntoResponse for CustomError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            CustomError::FaultySetup(message) => (StatusCode::UNPROCESSABLE_ENTITY, message),
            CustomError::Database(message) => (StatusCode::UNPROCESSABLE_ENTITY, message),
        };
        format!("status = {}, message = {}", status, error_message).into_response()
    }
}

impl From<axum::http::uri::InvalidUri> for CustomError {
    fn from(err: axum::http::uri::InvalidUri) -> Self {
        CustomError::FaultySetup(err.to_string())
    }
}

impl From<PoolError> for CustomError {
    fn from(err: PoolError) -> Self {
        CustomError::Database(err.to_string())
    }
}

impl From<TokioPostgresError> for CustomError {
    fn from(err: TokioPostgresError) -> Self {
        CustomError::Database(err.to_string())
    }
}
