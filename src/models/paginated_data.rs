use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct PaginatedData<T> {
    pub from: usize,
    pub to: usize,
    pub last_page: usize,
    pub per_page: usize,
    pub page: usize,
    pub total: usize,
    pub data: Vec<T>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct PaginationQuery {
    pub page: Option<usize>,
    pub per_page: Option<usize>,
}
