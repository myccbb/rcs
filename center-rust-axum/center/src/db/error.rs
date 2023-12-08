#[derive(Debug, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("db error")]
    DBErr,
    #[error("record not found")]
    RecordNotFound,
    #[error("duplicate entry")]
    DuplicateEntry,
}

impl Error {
    pub fn from_sqlite(e: &sqlite::Error) -> Self {
        log::info!("sqlite error: {:?}", e);
        match e {
            sqlite::Error::ConstraintPrimaryKey | sqlite::Error::ConstraintUnique => {
                Error::DuplicateEntry
            }
            _ => Error::DBErr,
        }
    }
}

impl std::convert::From<&sqlite::Error> for Error {
    fn from(e: &sqlite::Error) -> Self {
        Error::from_sqlite(e)
    }
}

impl std::convert::From<sqlite::Error> for Error {
    fn from(e: sqlite::Error) -> Self {
        Error::from_sqlite(&e)
    }
}
