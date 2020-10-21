use std::{fmt, result};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    ETagError,
    SQLError(diesel::result::Error),
}

impl From<diesel::result::Error> for Error {
    fn from(e: diesel::result::Error) -> Self {
        Error::SQLError(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = match self {
            Error::ETagError => "ETag mismatch".to_owned(),
            Error::SQLError(e) => e.to_string(),
        };

        write!(f, "{}", output)
    }
}

impl Error {
    pub fn into_inner(&self) -> diesel::result::Error {
        match self {
            Error::ETagError => diesel::result::Error::NotFound,
            Error::SQLError(diesel::result::Error::RollbackTransaction) => {
                diesel::result::Error::RollbackTransaction
            }
            Error::SQLError(_) => diesel::result::Error::NotFound,
        }
    }
}
