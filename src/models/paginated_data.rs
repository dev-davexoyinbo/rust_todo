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

impl<T> PaginatedData<T> {
    pub fn from_data(data: Vec<T>, query: PaginationQuery, total: usize) -> PaginatedData<T> {
        let page = query.page.unwrap_or(1);
        let per_page = query.per_page.unwrap_or(15);

        let from = (page - 1) * per_page + if data.len() > 0 { 1 } else { 0 };
        let to = from + data.len();
        let last_page = (total as f32 / per_page as f32).floor() as usize + 1;
        return PaginatedData::<T> {
            from,
            to,
            last_page,
            per_page,
            page,
            total,
            data,
        };
    }
}

#[derive(Deserialize, Debug, Serialize)]
pub struct PaginationQuery {
    pub page: Option<usize>,
    pub per_page: Option<usize>,
}
