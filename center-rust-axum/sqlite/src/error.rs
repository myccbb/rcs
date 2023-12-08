#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    OpenConnectionFail(i32, String, Option<String>),
    PrepareFail(i32, String),
    BindFail(i32, String),
    StepFail(i32, String),
    ConstraintPrimaryKey,
    ConstraintUnique,
    InvalidSql(String),
    InvalidColumnType(i32),
    GetColumnFail(i32, i32, String),
    TextIsNotUtf8(String, String),
}

impl From<std::ffi::NulError> for Error {
    fn from(_: std::ffi::NulError) -> Self {
        Error::InvalidSql("0 byte is not allowed in sql string.".into())
    }
}
