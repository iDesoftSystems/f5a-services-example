use axum::{http::StatusCode, response::IntoResponse};
use sea_orm::DbErr;

pub enum ApiError {
    Unexpected(Box<dyn std::error::Error + Send + Sync>),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ApiError::Unexpected(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
        }
    }
}

impl From<DbErr> for ApiError {
    fn from(value: DbErr) -> Self {
        Self::Unexpected(Box::new(value))
    }
}
