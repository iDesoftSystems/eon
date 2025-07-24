use sea_orm::sqlx::error::ErrorKind;
use sea_orm::sqlx::Error;
use sea_orm::{DbErr, RuntimeErr};
use std::fmt::Display;

pub enum DataError {
    Internal(String),

    UniqueViolation,

    ForeignKeyViolation,
}

impl From<DbErr> for DataError {
    fn from(err: DbErr) -> Self {
        match err {
            DbErr::Exec(RuntimeErr::SqlxError(Error::Database(database_err))) => {
                match database_err.kind() {
                    ErrorKind::UniqueViolation => DataError::UniqueViolation,
                    ErrorKind::ForeignKeyViolation => DataError::ForeignKeyViolation,
                    _ => DataError::Internal(database_err.message().to_string()),
                }
            }
            _ => DataError::Internal(err.to_string()),
        }
    }
}

impl Display for DataError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let message = match self {
            DataError::Internal(msg) => msg,
            DataError::UniqueViolation => "UniqueViolation",
            DataError::ForeignKeyViolation => "ForeignKeyViolation",
        };

        write!(f, "{message}")
    }
}
