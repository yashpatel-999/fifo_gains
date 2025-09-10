use chrono::NaiveDate;
use rust_decimal::Decimal;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct TxRow {
    pub id: i32,
    pub date: NaiveDate,
    pub product: String,
    pub txn_type: String,
    pub quantity: i32,
    pub price: Decimal,   
}

#[derive(Debug)]
pub enum TxnType {
    Buy,
    Sell,
}

#[derive(Debug)]
pub struct Transaction {
    pub date: NaiveDate,
    pub product: String,
    pub txn_type: TxnType,
    pub quantity: u32,
    pub price: Decimal, 
}
