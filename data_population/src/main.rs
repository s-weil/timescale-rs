use chrono::Duration;
use common::{init_tracing, InsertableStockDefinition, StockDefinition};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::types::chrono::NaiveDate;
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use stopwatch::Stopwatch;

// TODO run db migrations -> sqlxcli
// TODO add cli to parse parameters for nr stocks, dates, etc

pub async fn init_postgres_pool(url: &str) -> Result<Pool<Postgres>, sqlx::Error> {
    let pool = PgPoolOptions::new().max_connections(8).connect(url).await?;
    Ok(pool)
}

fn create_sample_date(n_prices: usize, end_date: NaiveDate) -> (Vec<NaiveDate>, Vec<f64>) {
    let dates: Vec<NaiveDate> = (0..n_prices as i64)
        .map(|d| end_date + Duration::days(-d))
        .collect();

    let close_prices: Vec<f64> = dates
        .iter()
        .enumerate()
        .map(|(d, _)| 0.0 + 0.00001 * d as f64)
        .collect(); // todo randomization based on stock idx;
    (dates, close_prices)
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    init_tracing();
    dotenv().ok();

    let n_stocks = 1_000;
    let n_prices = 2_000;

    let database_url =
        std::env::var("DATABASE_URL").expect("expected .env variable `DATABASE_URL`");

    let pool = init_postgres_pool(&database_url).await?;

    tracing::debug!("Established connection pool");

    // test connection
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;
    assert_eq!(row.0, 150);

    let stocks: Vec<InsertableStockDefinition> = (0..n_stocks)
        .map(|idx| InsertableStockDefinition::new(idx.to_string()))
        .collect();

    for stock in stocks {
        let _result = sqlx::query("INSERT INTO stocks.stock_definitions (ticker) VALUES ($1);")
            .bind(stock.ticker)
            .execute(&pool)
            .await;
        // tracing::debug!("Insert success: {}", result.is_ok());
    }

    let stock_registry: Vec<StockDefinition> =
        sqlx::query_as!(StockDefinition, "SELECT * FROM stocks.stock_definitions")
            .fetch_all(&pool)
            .await?;

    tracing::debug!("loaded {} stocks", stock_registry.len());

    let stock_registry = Arc::new(stock_registry);

    let end_date = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
    let (dates, prices) = create_sample_date(n_prices, end_date);

    let date_grid = Arc::new(dates);
    let prices = Arc::new(prices);

    let _ = tokio::join!(
        timescale_population(
            stock_registry.clone(),
            date_grid.clone(),
            prices.clone(),
            &pool
        ),
        timeseries_population(
            stock_registry.clone(),
            date_grid.clone(),
            prices.clone(),
            &pool
        ),
    );

    tracing::debug!("Finished");

    Ok(())
}

async fn timeseries_population(
    stock_registry: Arc<Vec<StockDefinition>>,
    date_grid: Arc<Vec<NaiveDate>>,
    prices: Arc<Vec<f64>>,
    pool: &Pool<Postgres>,
) -> Result<(), sqlx::Error> {
    let sw = Stopwatch::start_new();
    for stock in stock_registry.iter() {
        let ids: Vec<i32> = date_grid.iter().map(|_| stock.id).collect();

        // https://www.alxolr.com/articles/rust-bulk-insert-to-postgre-sql-using-sqlx
        let _result = sqlx::query(
            "INSERT INTO stocks.stock_timeseries(stock_id, dt, close)\
             SELECT * FROM UNNEST($1::INTEGER[], $2::DATE[], $3::NUMERIC[])",
        )
        .bind(&ids)
        .bind(date_grid.as_ref())
        .bind(prices.as_ref())
        .execute(pool)
        .await;
    }

    tracing::debug!(ms = sw.elapsed_ms(), "populated stock_timescale");
    Ok(())
}

async fn timescale_population(
    stock_registry: Arc<Vec<StockDefinition>>,
    date_grid: Arc<Vec<NaiveDate>>,
    prices: Arc<Vec<f64>>,
    pool: &Pool<Postgres>,
) -> Result<(), sqlx::Error> {
    let sw = Stopwatch::start_new();
    for stock in stock_registry.iter() {
        let ids: Vec<i32> = date_grid.iter().map(|_| stock.id).collect();

        // https://www.alxolr.com/articles/rust-bulk-insert-to-postgre-sql-using-sqlx
        let _result = sqlx::query(
            "INSERT INTO stocks.stock_timescale(stock_id, dt, close)\
             SELECT * FROM UNNEST($1::INTEGER[], $2::DATE[], $3::NUMERIC[])",
        )
        .bind(&ids)
        .bind(date_grid.as_ref())
        .bind(prices.as_ref())
        .execute(pool)
        .await;
    }

    tracing::debug!(ms = sw.elapsed_ms(), "populated stock_timescale");
    Ok(())
}
