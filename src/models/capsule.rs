use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Capsule {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub unlock_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub opened: bool,
}

impl Capsule {
    pub fn is_unlocked(&self, now: DateTime<Utc>) -> bool {
        now >= self.unlock_date
    }
}
