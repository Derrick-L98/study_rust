use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryDto {
    pub account_id: i64,
    pub start_date: i64,
    pub end_date: i64,
}
