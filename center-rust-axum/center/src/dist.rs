pub mod cfg {
    use serde::{Deserialize, Serialize};

    #[derive(clap::Parser, Debug)]
    #[clap(author, version, about, long_about=None)]
    pub struct Args {
        #[clap(short, long, default_value = "config.toml")]
        pub conf: String,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Config {
        pub db: DBConfig,
        pub auth: AuthConfig,
    }

    impl Config {
        pub fn new(cfg_str: &str) -> Result<Self, toml::de::Error> {
            let mut cfg = toml::from_str::<Config>(cfg_str)?;
            cfg.auth.expire_duration = time::Duration::new(cfg.auth.expire_hours * 3600, 0);
            Ok(cfg)
        }
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct DBConfig {
        // pub name: String,
        // pub host: String,
        // pub port: u16,
        // pub username: String,
        // pub password: String,
        pub db_name: String,
    }
    impl DBConfig {
        // pub fn dsn(&self) -> String {
        //     format!(
        //         "mysql://{}:{}@{}:{}/{}",
        //         self.username, self.password, self.host, self.port, self.db_name
        //     )
        // }
    }
    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct AuthConfig {
        pub expire_hours: i64,
        #[serde(skip)]
        pub expire_duration: time::Duration,
    }
}

pub mod utils {
    use chrono::{DateTime, FixedOffset, SubsecRound, TimeZone, Utc};
    pub fn now_beijing() -> DateTime<FixedOffset> {
        let offset = FixedOffset::east_opt(8 * 3600).unwrap();
        offset.from_utc_datetime(&Utc::now().naive_utc().trunc_subsecs(0))
    }
    pub fn now_utc() -> DateTime<Utc> {
        Utc::now()
    }

    const ALPHABET_TABLE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const NUMERIC_TABLE: &str = "0123456789";
    const NUMERIC_ALPHABET_TABLE: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    pub fn random_alphabet_string(length: u8) -> String {
        random_string(ALPHABET_TABLE, length)
    }

    pub fn random_numeric_string(length: u8) -> String {
        random_string(NUMERIC_TABLE, length)
    }

    pub fn random_numeric_alphabet_string(length: u8) -> String {
        random_string(NUMERIC_ALPHABET_TABLE, length)
    }

    fn random_string(table: &str, length: u8) -> String {
        let mut s = String::new();
        for _i in 0..length {
            s.push(
                table
                    .chars()
                    .nth(rand::random::<usize>() % table.len())
                    .unwrap(),
            )
        }
        s
    }
}

pub mod api {
    use serde::{self, Deserialize, Serialize};
    use serde_repr::{self, Deserialize_repr, Serialize_repr};

    pub fn success<T>(data: T) -> Res<T>
    where
        T: Serialize,
    {
        Res {
            code: Code::Success,
            msg: String::from("success"),
            data,
        }
    }
    pub fn response<T>(code: Code, msg: String, data: T) -> Res<T>
    where
        T: Serialize,
    {
        Res { code, msg, data }
    }

    #[derive(Clone)]
    pub struct ServerContext {
        pub center_conn: sqlite::Connection,
    }

    #[derive(Debug, Clone)]
    pub struct AddLoginUserError;

    impl ServerContext {
        pub fn new(center_conn: sqlite::Connection) -> Self {
            ServerContext { center_conn }
        }
    }

    #[derive(Serialize, Deserialize)]
    pub struct LoginReq {
        pub name: String,
        pub password: String,
    }
    #[derive(Serialize, Deserialize)]
    pub struct LoginRes {
        pub name: String,
        pub token: String,
        pub expire_time: String,
    }

    #[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
    #[repr(i32)]
    pub enum Code {
        Success = 0,
        SysError = 1,
        InvalidParam = 2,
        DBError = 3,
        LoginError = 4,
        AuthError = 5,
        ObjectNotFound = 6,
        ObjectExist = 7,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Res<T>
    where
        T: Serialize,
    {
        pub code: Code,
        pub msg: String,
        pub data: T,
    }

    #[derive(Serialize, Deserialize)]
    pub struct List<T> {
        pub page: u32,
        pub page_size: usize,
        pub total: u64,
        pub results: Vec<T>,
    }
}
