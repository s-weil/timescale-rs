use chrono::Duration;
use common::{InsertableStockDefinition, StockDefinition};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::types::chrono::NaiveDate;

// TODO run db migrations -> sqlxcli
// TODO add cli to parse parameters for nr stocks, dates, etc

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let database_url =
        std::env::var("DATABASE_URL").expect("expected .env variable `DATABASE_URL`");

    let pool = PgPoolOptions::new()
        .max_connections(8)
        .connect(&database_url)
        .await?;

    println!("Established connection pool to {database_url}");

    // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL/MariaDB)
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;

    assert_eq!(row.0, 150);

    // let connection = pool.acquire().await?;

    let stocks: Vec<InsertableStockDefinition> = (0..100)
        .into_iter()
        .map(|idx| InsertableStockDefinition::new(idx.to_string()))
        .collect();

    for stock in stocks {
        let result = sqlx::query("INSERT INTO stocks.stock_definitions (ticker) VALUES ($1);")
            .bind(stock.ticker)
            // .bind(data.customer_id)
            .execute(&pool)
            .await;
        println!("Insert success: {}", result.is_ok());
    }

    let stocks2: Vec<StockDefinition> =
        sqlx::query_as!(StockDefinition, "SELECT * FROM stocks.stock_definitions")
            .fetch_all(&pool)
            .await?;

    println!("loaded {} stocks", stocks2.len());

    // TODO bulk insert for stock prices

    let end_date = NaiveDate::from_ymd_opt(2024, 12, 12).unwrap();
    let dates: Vec<NaiveDate> = (0..1000)
        .into_iter()
        .map(|d| end_date.clone() + Duration::days(-d))
        .collect();
    let close_prices: Vec<f64> = (0..1000)
        .into_iter()
        .map(|d| 0.0 + 0.00001 * d as f64)
        .collect(); // todo randomization based on stock idx;
    let ids: Vec<i32> = (0..1000).into_iter().map(|d| 1).collect();

    // https://www.alxolr.com/articles/rust-bulk-insert-to-postgre-sql-using-sqlx
    let result = sqlx::query(
        "INSERT INTO stocks.stock_timeseries(stock_id, dt, close) SELECT * FROM UNNEST($1::INTEGER[], $2::DATE[], $3::NUMERIC[])")
        .bind(&ids).bind(&dates).bind(&close_prices)
        .execute(&pool)
        .await;

    println!("bulk inserted close prices {:?}", result);

    println!("Finished");

    Ok(())
}
