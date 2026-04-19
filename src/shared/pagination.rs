use serde::Deserialize;
use serde::Serialize;
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, IntoParams, ToSchema, Serialize)]
pub struct Pagination {
    #[schema(example = 1)]
    // Page number (starts at 0)
    pub page: u64,

    #[schema(example = 10)]
    /// Number of items per page
    pub page_size: u64,
}
