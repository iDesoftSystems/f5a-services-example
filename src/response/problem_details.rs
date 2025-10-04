use serde::Serialize;

use crate::response::Field;

#[derive(Serialize)]
pub struct ProblemDetails {
    pub detail: String,
    pub errors: Vec<Field>,
}
