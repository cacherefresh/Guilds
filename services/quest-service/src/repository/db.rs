use std::fmt;
use tokio_postgres::Error as PgError;

#[derive(Debug)]
pub enum DbError {
    PgError(PgError),
    NoDataReturned,
    Other(String),
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DbError::PgError(e) => write!(f, "Database error: {}", e),
            DbError::NoDataReturned => write!(f, "No data returned"),
            DbError::Other(msg) => write!(f, "Database error: {}", msg),
        }
    }
}

impl std::error::Error for DbError {}

impl From<PgError> for DbError {
    fn from(error: PgError) -> Self {
        DbError::PgError(error)
    }
}

impl From<String> for DbError {
    fn from(error: String) -> Self {
        DbError::Other(error)
    }
} 