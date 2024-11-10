use chrono::NaiveDate;
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct InsertableStockDefinition {
    pub ticker: String,
}

#[derive(Debug, FromRow, Serialize)]
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
    pub dt: NaiveDate,
    pub close: bigdecimal::BigDecimal,
}

#[derive(Debug, Serialize)]
pub struct Price {
    pub dt: NaiveDate,
    pub v: bigdecimal::BigDecimal,
}

impl From<StockPrice> for Price {
    fn from(stock_price: StockPrice) -> Self {
        Self {
            v: stock_price.close,
            dt: stock_price.dt, //.to_string(),
        }
    }
}
