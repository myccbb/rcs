use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Code {
    Success = 0,
    InitError = 1,
    SystemError = 2,
    DBError = 3,
}
