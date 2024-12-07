use chrono::{Duration, NaiveDate};
use common::StockDefinition;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::sync::Arc;

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

pub async fn bench_setup(
    n_prices: usize,
) -> Result<
    (
        Pool<Postgres>,
        Arc<Vec<StockDefinition>>,
        Arc<Vec<NaiveDate>>,
        Arc<Vec<f64>>,
    ),
    sqlx::Error,
> {
    dotenv().ok();
    let database_url =
        std::env::var("DATABASE_URL").expect("expected .env variable `DATABASE_URL`");

    let pool = init_postgres_pool(&database_url).await?;

    let stock_registry: Vec<StockDefinition> =
        sqlx::query_as!(StockDefinition, "SELECT * FROM stocks.stock_definitions")
            .fetch_all(&pool)
            .await?;

    let end_date = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
    let (dates, prices) = create_sample_date(n_prices, end_date);

    let stock_registry = Arc::new(stock_registry);
    let date_grid = Arc::new(dates);
    let prices = Arc::new(prices);
    Ok((pool, stock_registry, date_grid, prices))
}

async fn timeseries_population(
    stock_registry: Arc<Vec<StockDefinition>>,
    date_grid: Arc<Vec<NaiveDate>>,
    prices: Arc<Vec<f64>>,
    pool: &Pool<Postgres>,
) -> Result<(), sqlx::Error> {
    // let sw = Stopwatch::start_new();
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

    // tracing::debug!(ms = sw.elapsed_ms(), "populated stock_timescale");
    Ok(())
}

async fn timescale_population(
    stock_registry: Arc<Vec<StockDefinition>>,
    date_grid: Arc<Vec<NaiveDate>>,
    prices: Arc<Vec<f64>>,
    pool: &Pool<Postgres>,
) -> Result<(), sqlx::Error> {
    // let sw = Stopwatch::start_new();
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

    Ok(())
}

pub async fn bench_timescale(n_prices: usize) -> Result<usize, sqlx::Error> {
    let (pool, stock_registry, date_grid, prices) = bench_setup(n_prices).await?;

    let _ = sqlx::query("TRUNCATE stocks.stock_timescale;")
        .execute(&pool)
        .await?;

    timescale_population(stock_registry, date_grid, prices, &pool).await?;
    Ok(n_prices)
}
