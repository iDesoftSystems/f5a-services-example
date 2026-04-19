use serde::Serialize;
use utoipa::ToSchema;

use crate::shared::response::Field;

#[derive(Serialize, ToSchema)]
#[schema(
    description = "Standard error response format for validation failures",
    example = json!({
        "detail": "Request validation failed",
        "errors": [
            { "field": "username", "reason": "must be unique", "code": "unique" },
            { "field": "email", "reason": "invalid format", "code": "format" }
        ]
    })
)]
pub struct ProblemDetails {
    #[schema(example = "Request validation failed")]
    pub detail: String,
    pub errors: Vec<Field>,
}
