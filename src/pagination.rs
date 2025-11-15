use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Deserialize, IntoParams)]
pub struct Pagination {
    pub page: u64,
    pub page_size: u64,
}
