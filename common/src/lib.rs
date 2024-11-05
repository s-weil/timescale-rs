use sqlx::types::chrono::{NaiveDate, NaiveDateTime};
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct InsertableStockDefinition {
    pub ticker: String,
}

#[derive(Debug, FromRow)]
pub struct StockDefinition {
    pub id: i32,
    pub ticker: String,
}

impl InsertableStockDefinition {
    pub fn new(ticker: String) -> Self {
        Self { ticker }
    }
}

#[derive(Debug, FromRow)]
pub struct StockPrice {
    pub stock_id: i32,
    pub dt: NaiveDateTime,
    pub close: f32,
}
