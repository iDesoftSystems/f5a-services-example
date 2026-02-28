use axum::{http::StatusCode, response::IntoResponse};
use sea_orm::DbErr;
use thiserror::Error;
use validator::ValidationErrors;

use crate::shared::response::{BadRequest, UnprocessableEntity};

#[derive(Error, Debug)]
pub enum ApiError {
    #[error(transparent)]
    Unexpected(Box<dyn std::error::Error + Send + Sync>),

    #[error("Not Found")]
    NotFound,

    #[error(transparent)]
    Validation(ValidationErrors),

    #[error("{0}")]
    UnprocessableEntity(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ApiError::Unexpected(err) => {
                tracing::error!(err = err, "internal server error");
                (StatusCode::INTERNAL_SERVER_ERROR).into_response()
            }
            ApiError::NotFound => (StatusCode::NOT_FOUND).into_response(),
            ApiError::Validation(errs) => BadRequest(errs).into_response(),
            ApiError::UnprocessableEntity(msg) => UnprocessableEntity(msg).into_response(),
        }
    }
}

impl From<DbErr> for ApiError {
    fn from(value: DbErr) -> Self {
        Self::Unexpected(Box::new(value))
    }
}

impl From<ValidationErrors> for ApiError {
    fn from(value: ValidationErrors) -> Self {
        Self::Validation(value)
    }
}
