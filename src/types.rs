use chrono::{DateTime, Utc};

pub type QuestionId = String;

pub fn to_db_time(value: DateTime<Utc>) -> String {
    value.to_rfc3339()
}

pub fn from_db_time(value: &str) -> crate::errors::Result<DateTime<Utc>> {
    Ok(DateTime::parse_from_rfc3339(value)?.with_timezone(&Utc))
}
