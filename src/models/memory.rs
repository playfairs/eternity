use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Memory {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub timestamp: DateTime<Utc>,
}
