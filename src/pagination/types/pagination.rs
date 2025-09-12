use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Pagination {
    pub page_size: u64,
    pub page: u64,
}
