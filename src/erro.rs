use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("database error: {0}")]
    Db(#[from] rusqlite::Error),

    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("date parse error: {0}")]
    DateParse(#[from] chrono::ParseError),

    #[error("no questions were loaded from {0}")]
    NoQuestions(String),

    #[error("{entity} not found: {id}")]
    NotFound {
        entity: &'static str,
        id: String,
    },

    #[error("invalid input: {0}")]
    InvalidInput(String),
}
