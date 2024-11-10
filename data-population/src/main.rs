use chrono::Duration;
use common::{InsertableStockDefinition, StockDefinition};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::types::chrono::NaiveDate;
use sqlx::{Pool, Postgres};
use std::error::Error;
use std::sync::Arc;
use stopwatch::Stopwatch;
use tracing_subscriber::layer::SubscriberExt;
// TODO run db migrations -> sqlxcli
// TODO add cli to parse parameters for nr stocks, dates, etc

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenv().ok();

    let n_stocks = 1_000;
    let n_prices = 2_000;

    let database_url =
        std::env::var("DATABASE_URL").expect("expected .env variable `DATABASE_URL`");

    let pool = PgPoolOptions::new()
        .max_connections(8)
        .connect(&database_url)
        .await?;

    tracing::debug!("Established connection pool to {database_url}");

    // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL/MariaDB)
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;

    assert_eq!(row.0, 150);

    let stocks: Vec<InsertableStockDefinition> = (0..n_stocks)
        // .into_iter()
        .map(|idx| InsertableStockDefinition::new(idx.to_string()))
        .collect();

    for stock in stocks {
        let _result = sqlx::query("INSERT INTO stocks.stock_definitions (ticker) VALUES ($1);")
            .bind(stock.ticker)
            // .bind(data.customer_id)
            .execute(&pool)
            .await;
        // println!("Insert success: {}", result.is_ok());
    }

    let stock_registry: Vec<StockDefinition> =
        sqlx::query_as!(StockDefinition, "SELECT * FROM stocks.stock_definitions")
            .fetch_all(&pool)
            .await?;

    tracing::debug!("loaded {} stocks", stock_registry.len());

    let stock_registry = Arc::new(stock_registry);

    let end_date = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
    let dates: Vec<NaiveDate> = (0..n_prices)
        // .into_iter()
        .map(|d| end_date + Duration::days(-d))
        .collect();
    let date_grid = Arc::new(dates);

    let close_prices: Vec<f64> = date_grid
        .iter()
        .enumerate()
        .map(|(d, _)| 0.0 + 0.00001 * d as f64)
        .collect(); // todo randomization based on stock idx;
    let prices = Arc::new(close_prices);

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
) -> Result<(), Box<dyn Error>> {
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

        // println!("{}:{}", stock.id, result.is_ok())
    }

    tracing::debug!("stock_timeseries within {} ms", sw.elapsed_ms());
    Ok(())
}

async fn timescale_population(
    stock_registry: Arc<Vec<StockDefinition>>,
    date_grid: Arc<Vec<NaiveDate>>,
    prices: Arc<Vec<f64>>,
    pool: &Pool<Postgres>,
) -> Result<(), Box<dyn Error>> {
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

        // println!("{}:{}", stock.id, result.is_ok())
    }

    tracing::debug!("stock_timescale within {} ms", sw.elapsed_ms());
    Ok(())
}
