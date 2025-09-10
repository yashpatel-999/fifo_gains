use std::collections::{HashMap, VecDeque};
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;

use crate::models::{Transaction, TxnType};

pub fn compute_fifo_pnl(transactions: &[Transaction]) -> Result<HashMap<String, Decimal>, String> {
    let mut lots: HashMap<String, VecDeque<(u32, Decimal)>> = HashMap::new();
    let mut pnl: HashMap<String, Decimal> = HashMap::new();

    for tx in transactions {
        let prod = tx.product.clone();
        match tx.txn_type {
            TxnType::Buy => {
                lots.entry(prod).or_default().push_back((tx.quantity, tx.price));
            }
            TxnType::Sell => {
                let mut qty_to_sell = tx.quantity;
                let queue = lots.entry(prod.clone()).or_default();
                let available: u64 = queue.iter().map(|(q, _)| *q as u64).sum();
                if qty_to_sell as u64 > available {
                    return Err(format!(
                        "Sell of {} units for '{}' on {} but only {} available",
                        qty_to_sell, prod, tx.date, available
                    ));
                }

                while qty_to_sell > 0 {
                    let front = queue.front_mut().unwrap();
                    let take = std::cmp::min(front.0, qty_to_sell);
                    let take_dec = Decimal::from_i64(take as i64).unwrap();
                    let realized = (tx.price - front.1) * take_dec;
                    let entry = pnl.entry(prod.clone()).or_insert(Decimal::ZERO);
                    *entry += realized;

                    front.0 -= take;
                    qty_to_sell -= take;
                    if front.0 == 0 {
                        queue.pop_front();
                    }
                }
            }
        }
    }

    Ok(pnl)
}
