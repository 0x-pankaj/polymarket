use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Event {
    pub id: String,
    pub name: String,
    pub description: String,
    pub created_at: i64,
    pub expires_at: i64,
}
