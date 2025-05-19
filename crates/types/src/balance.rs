use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Balance {
    pub user_id: String,
    pub available: f64,
    pub locked: f64,
}
