mod models;
mod fifo;
mod db;

use std::env;
use std::error::Error;
use sqlx::PgPool;
use rust_decimal::prelude::*;
use dotenvy::dotenv;  
use fifo::compute_fifo_pnl;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
 
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = PgPool::connect(&database_url).await?;

    let txs = db::fetch_transactions(&pool).await?;
    let pnl_map = compute_fifo_pnl(&txs).map_err(|e| format!("Compute error: {}", e))?;

    println!("P/L per product (realized, FIFO):");
    for (product, pnl) in &pnl_map {
        sqlx::query(
            "INSERT INTO pnl_results (product, pnl, computed_at)
             VALUES ($1, $2, now())
             ON CONFLICT (product) DO UPDATE
               SET pnl = EXCLUDED.pnl, computed_at = now()",
        )
        .bind(product)
        .bind(pnl) 
        .execute(&pool)
        .await?;

        println!("{}: {:.2}", product, pnl.to_f64().unwrap());
    }

    Ok(())
}
