use axum::{Json, http::StatusCode, response::IntoResponse};

use crate::response::ProblemDetails;

pub struct UnprocessableEntity(pub String);

impl IntoResponse for UnprocessableEntity {
    fn into_response(self) -> axum::response::Response {
        let problem_details = ProblemDetails {
            detail: self.0,
            errors: Vec::new(),
        };

        (StatusCode::UNPROCESSABLE_ENTITY, Json(problem_details)).into_response()
    }
}
