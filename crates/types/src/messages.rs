use crate::order::{OptionType, Order, OrderType};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum MessageFromApi {
    CreateOrder {
        market: String,
        price: f64,
        quantity: u32,
        side: OrderType,
        option: OptionType,
        user_id: String,
    },
    CancelOrder {
        order_id: String,
        market: String,
        option: OptionType,
    },
    OnRamp {
        amount: f64,
        user_id: String,
        txn_id: String,
    },
    GetDepth {
        market: String,
        option: OptionType,
    },
    GetOpenOrders {
        user_id: String,
        market: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum MessageToApi {
    Depth { payload: DepthPayload },
    OrderPlaced { payload: OrderPlacedPayload },
    OrderCancelled { payload: OrderCancelledPayload },
    OpenOrders { payload: Vec<Order> },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DepthPayload {
    pub bids: Vec<(f64, u32)>,
    pub asks: Vec<(f64, u32)>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OrderPlacedPayload {
    pub order_id: String,
    pub executed_qty: u32,
    pub fills: Vec<Fill>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Fill {
    pub price: f64,
    pub qty: u32,
    pub trade_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OrderCancelledPayload {
    pub order_id: String,
    pub executed_qty: u32,
    pub remaining_qty: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TickerUpdateMessage {
    pub stream: String,
    pub data: TickerData,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TickerData {
    pub c: Option<f64>, // current price
    pub h: Option<f64>, // high
    pub l: Option<f64>, // low
    pub v: Option<u32>, // volume
    pub s: String,      // symbol
    pub id: u64,
    #[serde(rename = "e")]
    pub event: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DepthUpdateMessage {
    pub stream: String,
    pub data: DepthData,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DepthData {
    pub b: Vec<(f64, u32)>, // bids
    pub a: Vec<(f64, u32)>, // asks
    #[serde(rename = "e")]
    pub event: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TradeAddedMessage {
    pub stream: String,
    pub data: TradeData,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TradeData {
    #[serde(rename = "e")]
    pub event: String,
    pub t: u64,    // trade id
    pub m: bool,   // market order
    pub p: f64,    // price
    pub q: u32,    // quantity
    pub s: String, // symbol
}
