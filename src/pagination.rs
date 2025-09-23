use serde::Deserialize;

#[derive(Deserialize)]
pub struct Pagination {
    pub page: u64,
    pub page_size: u64,
}
