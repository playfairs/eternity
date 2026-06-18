use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::QuestionId;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Question {
    pub id: QuestionId,
    pub category: String,
    pub prompt: String,
    pub created_at: DateTime<Utc>,
}

impl Question {
    pub fn new(id: QuestionId, category: impl Into<String>, prompt: impl Into<String>) -> Self {
        Self {
            id,
            category: category.into(),
            prompt: prompt.into(),
            created_at: Utc::now(),
        }
    }
}
