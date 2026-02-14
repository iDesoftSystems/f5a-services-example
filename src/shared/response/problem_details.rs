use serde::Serialize;
use utoipa::ToSchema;

use crate::shared::response::Field;

#[derive(Serialize, ToSchema)]
pub struct ProblemDetails {
    pub detail: String,
    pub errors: Vec<Field>,
}
