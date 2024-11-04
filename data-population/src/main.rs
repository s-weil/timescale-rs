use common::InsertableStockDefinition;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

// TODO run db migrations
// TODO add cli to parse parameters for nr stocks, dates, etc

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let database_url =
        std::env::var("DATABASE_URL").expect("expected .env variable `DATABASE_URL`");

    let pool = PgPoolOptions::new()
        .max_connections(5)
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

    // // TODO bulk insert
    // sqlx::query!(
    //     "INSERT INTO stock_definitions(ticker) SELECT * FROM UNNEST($1::text[])",
    //     &stocks[..]
    // )
    // .execute(&pool)
    // .await
    // .unwrap();

    println!("Finished");

    Ok(())
}
