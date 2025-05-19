use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum OptionType {
    Yes,
    No,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum OrderType {
    Buy,
    Sell,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Order {
    pub id: String,
    pub user_id: String,
    pub market: String,
    pub option: OptionType,
    pub order_type: OrderType,
    pub price: f64,
    pub quantity: u32,
    pub timestamp: i64,
}
