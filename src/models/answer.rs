use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::QuestionId;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Answer {
    pub id: i64,
    pub question_id: QuestionId,
    pub content: String,
    pub timestamp: DateTime<Utc>,
}
