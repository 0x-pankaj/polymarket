use crate::order::OptionType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Trade {
    pub buy_order_id: String,
    pub sell_order_id: String,
    pub market: String,
    pub option: OptionType,
    pub price: f64,
    pub quantity: u32,
    pub timestamp: i64,
}
