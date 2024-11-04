pub struct InsertableStockDefinition {
    pub ticker: String,
}

impl InsertableStockDefinition {
    pub fn new(ticker: String) -> Self {
        Self { ticker }
    }
}
