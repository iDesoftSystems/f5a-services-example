use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct Field {
    #[schema(example = "username")]
    pub field: String,

    #[schema(example = "must be unique")]
    pub reason: String,

    #[schema(example = "unique")]
    pub code: String,
}

impl Field {
    pub fn new(field: &str, reason: &str, code: &str) -> Self {
        Self {
            field: field.to_string(),
            reason: reason.to_string(),
            code: code.to_string(),
        }
    }
}
