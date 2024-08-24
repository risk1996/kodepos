use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiListResponse<T> {
  pub data: Vec<T>,
  pub element_count: usize,
  pub page: usize,
  pub size: usize,
  pub total: usize,
  pub page_count: usize,
  pub is_first: bool,
  pub is_last: bool,
}

impl<T> ApiListResponse<T> {
  pub fn new(data: Vec<T>, page: usize, size: usize, total: usize) -> Self {
    let element_count = data.len();
    let page_count = (total as f64 / size as f64).ceil() as usize;
    let is_first = page == 0;
    let is_last = page_count.saturating_sub(1).eq(&page);

    Self {
      data,
      element_count,
      page,
      size,
      total,
      page_count,
      is_first,
      is_last,
    }
  }
}
