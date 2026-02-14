use std::borrow::Cow;

use axum::{Json, http::StatusCode, response::IntoResponse};
use validator::{ValidationError, ValidationErrors};

use crate::shared::response::{Field, ProblemDetails};

const INVALID_DEFAULT_MESSAGE: Cow<'static, str> = Cow::Borrowed("Invalid information");

trait IntoFields {
    fn into_fields(self) -> Vec<Field>;
}

pub struct BadRequest(pub ValidationErrors);

impl IntoResponse for BadRequest {
    fn into_response(self) -> axum::response::Response {
        let fields_with_errors = self.0.into_fields();

        let problem_details = ProblemDetails {
            detail: String::from("Validation failed"),
            errors: fields_with_errors,
        };
        (StatusCode::BAD_REQUEST, Json(problem_details)).into_response()
    }
}

impl IntoFields for ValidationErrors {
    fn into_fields(self) -> Vec<Field> {
        let mut fields: Vec<Field> = Vec::with_capacity(self.field_errors().len());

        let field_errors = self.field_errors();

        fields.extend(field_errors.into_iter().map(|(field_name, errs)| {
            let error: &ValidationError = &errs[0];
            let field_message = error.message.as_ref().unwrap_or(&INVALID_DEFAULT_MESSAGE);

            Field::new(&field_name, field_message, &error.code)
        }));

        fields.sort_by(|a, b| a.field.to_lowercase().cmp(&b.field.to_lowercase()));

        fields
    }
}
