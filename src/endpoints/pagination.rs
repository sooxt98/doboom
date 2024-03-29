use std::default::Default;

const DEFAULT_PER_PAGE: i32 = 10;
const DEFAULT_PAGE: i32 = 1;

pub struct Pagination {
    per_page: Option<i32>,
    page: Option<i32>
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            per_page: Some(DEFAULT_PER_PAGE),
            page: Some(DEFAULT_PAGE)
        }
    }
}

impl Pagination {
    pub fn get_per_page(&self) -> i32 {
        self.per_page.unwrap_or(DEFAULT_PER_PAGE)
    }

    pub fn get_page(&self) -> i32 {
        self.page.unwrap_or(DEFAULT_PAGE)
    }
}

