use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Profile {
    pub first_run: bool,
    pub total_days: i64,
    pub total_answers: i64,
    pub total_memories: i64,
    pub total_journal_entries: i64,
    pub total_capsules: i64,
}

