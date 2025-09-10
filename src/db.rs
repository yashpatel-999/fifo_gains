use sqlx::PgPool;
use crate::models::{TxRow, Transaction, TxnType};

pub async fn fetch_transactions(pool: &PgPool) -> Result<Vec<Transaction>, sqlx::Error> {
    let rows: Vec<TxRow> = sqlx::query_as::<_, TxRow>(
        "SELECT id, date, product, txn_type, quantity, price
         FROM transactions
         ORDER BY date ASC, id ASC",
    )
    .fetch_all(pool)
    .await?;

    let mut txs = Vec::with_capacity(rows.len());
    for r in rows {
        let ttype = match r.txn_type.to_lowercase().as_str() {
            s if s.contains("buy") => TxnType::Buy,
            s if s.contains("sell") => TxnType::Sell,
            _ => continue,
        };
        txs.push(Transaction {
            date: r.date,
            product: r.product,
            txn_type: ttype,
            quantity: r.quantity as u32,
            price: r.price,   // ✅ Decimal
        });
    }
    Ok(txs)
}
