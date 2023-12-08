#[derive(Debug, thiserror::Error)]
pub enum Error {
    DB(crate::db::Error),
    SerdeJson(serde_json::Error),
    Unknown,
}

impl std::convert::From<sqlite::Error> for Error {
    fn from(e: sqlite::Error) -> Self {
        Self::DB(e.into())
    }
}

impl std::convert::From<crate::db::Error> for Error {
    fn from(e: crate::db::Error) -> Self {
        Self::DB(e)
    }
}

impl std::convert::From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::SerdeJson(e)
    }
}

impl std::convert::From<anyhow::Error> for Error {
    fn from(_: anyhow::Error) -> Self {
        Self::Unknown
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Error::DB(e) => write!(f, "DBError: {}", e),
            Error::SerdeJson(e) => write!(f, "SerdeJsonError: {}", e),
            Error::Unknown => write!(f, "UnknownError"),
        }
    }
}
