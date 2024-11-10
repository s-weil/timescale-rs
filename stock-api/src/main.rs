use axum::extract::Query;
use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use common::StockDefinition;
use dotenv::dotenv;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::error::Error;
use std::time::Duration;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenv().ok();
    let database_url =
        std::env::var("DATABASE_URL").expect("expected .env variable `DATABASE_URL`");

    let pool = PgPoolOptions::new()
        .max_connections(8)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&database_url)
        .await?;

    // build our application with some routes
    let app = Router::new()
        .route("/", get(using_connection_pool_extractor))
        // .route("/stocks", get(stock_registry))
        .route("/stocks", get(stock))
        .with_state(pool);

    let listener = TcpListener::bind("0.0.0.0:8000").await?;
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .await
        .expect("Server start on port 8000");
    Ok(())
}

async fn using_connection_pool_extractor(
    State(pool): State<PgPool>,
) -> Result<String, (StatusCode, String)> {
    sqlx::query_scalar("select 'hello world from pg'")
        .fetch_one(&pool)
        .await
        .map_err(internal_error)
}

// TODO use DTO
async fn stock_registry(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<StockDefinition>>, (StatusCode, String)> {
    let registry: Vec<StockDefinition> =
        sqlx::query_as!(StockDefinition, "SELECT * FROM stocks.stock_definitions")
            .fetch_all(&pool)
            .await
            .map_err(internal_error)?;

    tracing::debug!("loaded {} stock definitions", registry.len());

    Ok(Json(registry))
}

// TODO use DTO
async fn stock(
    State(pool): State<PgPool>,
    Query(stock_id): Query<i32>,
) -> Result<Json<StockDefinition>, (StatusCode, String)> {
    let stock = sqlx::query_as("SELECT * FROM stocks.stock_definitions WHERE id = $1")
        .bind(stock_id)
        .fetch_one(&pool)
        .await
        .map_err(internal_error)?;

    Ok(Json(stock))
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E: Error>(err: E) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
