use sqlite;
use serde::{Serialize, Deserialize};
use once_cell::sync::OnceCell;


#[derive(Serialize, Deserialize)]
enum ErrorCode {
    InitError = 1,
    SystemError = 2,
    DBError = 3,
}

#[derive(Serialize, Deserialize)]
pub struct Error {
    code: ErrorCode,
    msg: String,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub path: String,
}

static CONFIG: OnceCell<Config> = OnceCell::new();


pub fn init(config: Config) -> Result<(), Error> {
    if let Err(_) = CONFIG.set(config) {
        return Err(Error {
            code: ErrorCode::InitError,
            msg: "failed to init simple_note config".to_string(),
        });
    }
    Ok(())
}

pub fn add_note() -> Result<String, Error> {
    let config = get_config()?;
    let conn = sqlite::open(config.path);
    if let Err(e) = conn {
        return Err(Error {
            code: ErrorCode::DBError,
            msg: "Unable to open database".to_string(),
        });
    }
    let conn = conn.unwrap();
    // let mut statement = conn.prepare()
    Ok("".to_string())
}


pub fn delete_note() {
    println!("delete_note");
}

fn get_config() -> Result<Config, Error> {
    if let Some(config) = CONFIG.get() {
        return Ok(config.clone());
    }
    return Err(Error {
        code: ErrorCode::InitError,
        msg: "simple_note not initialized".to_string(),
    });
}
