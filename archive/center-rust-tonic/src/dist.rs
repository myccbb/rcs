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
        pub name: String,
    }
    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct AuthConfig {
        pub expire_hours: i64,
        #[serde(skip)]
        pub expire_duration: time::Duration,
    }
}


/*
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
            data: data,
        }
    }
    pub fn response<T>(code: Code, msg: String, data: T) -> Res<T>
    where
        T: Serialize,
    {
        Res {
            code: code,
            msg: msg,
            data: data,
        }
    }

    use crate::dist::cfg::Config;
    use axum::extract;
    use std::{collections::HashMap, sync};
    use time::{self, ext::NumericalDuration};

    #[derive(Serialize, Deserialize)]
    pub struct LoginUser {
        name: String,
        token: String,
        login_time: time::OffsetDateTime,
        expire_time: time::OffsetDateTime,
    }

    impl LoginUser {
        pub fn new(
            name: &str,
            token: &str,
            login_time: time::OffsetDateTime,
            expire_duration: time::Duration,
        ) -> Self {
            LoginUser {
                name: String::from(name),
                token: String::from(token),
                login_time,
                expire_time: login_time + expire_duration,
            }
        }
    }

    pub struct ServerContext {
        pub login_user: HashMap<String, LoginUser>,
    }

    impl ServerContext {
        pub fn new() -> Self {
            ServerContext {
                login_user: HashMap::new(),
            }
        }
        pub fn add_login_user(&mut self, login_user: LoginUser) {
            self.login_user.insert(login_user.name.clone(), login_user);
        }
    }

    use crate::db;
    use tracing as log;

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

    fn truncate_to_millisecond(t: time::OffsetDateTime) -> time::OffsetDateTime {
        return t.saturating_sub(time::Duration::nanoseconds(t.nanosecond() as i64 - t.millisecond() as i64 * 1000_000))
    }

    pub async fn login(
        server_context: extract::Extension<sync::Arc<sync::RwLock<ServerContext>>>,
        cfg: extract::Extension<Config>,
        req: extract::Json<LoginReq>,
    ) -> axum::Json<Res<Option<LoginRes>>> {
        let conn = rusqlite::Connection::open(cfg.db.name.as_str());
        if let Err(e) = conn {
            return axum::Json(response(Code::DBError, e.to_string(), None));
        }
        let conn = conn.unwrap();
        let user = db::User::get_by_name(&conn, req.name.as_str());
        if let Err(e) = user {
            log::info!("{}", e.to_string());
            return axum::Json(response(Code::DBError, e.to_string(), None));
        }
        drop(conn);

        let user = user.unwrap();
        if user.is_none() {
            log::info!("user not found, reject login, {}", req.name);
            return axum::Json(response(Code::LoginError, "login failed".into(), None));
        }

        let server_context = server_context.write();
        if let Err(e) = server_context {
            log::info!("failed to get server_context write lock, {}", e);
            return axum::Json(response(Code::SysError, "login failed".into(), None));
        }
        let mut server_context = server_context.unwrap();
        let now = truncate_to_millisecond(time::OffsetDateTime::now_utc());
        let token = format!(
            "{}|{}",
            req.password,
            now.format(&time::format_description::well_known::Rfc3339)
                .unwrap()
        );
        let token_hash = ring::digest::digest(&ring::digest::SHA256, token.as_bytes());
        let token_bytes = token_hash.as_ref();
        log::info!("{}", data_encoding::HEXLOWER.encode(token_bytes));
        let login_user = LoginUser::new(&req.name, &token, now, cfg.auth.expire_duration);
        let login_res = LoginRes {
            name: req.name.clone(),
            token,
            expire_time: login_user
                .expire_time
                .format(&time::format_description::well_known::Rfc3339)
                .unwrap(),
        };
        server_context.add_login_user(login_user);
        drop(server_context);
        axum::Json(success(Some(login_res)))
    }

    fn verify_password(passwd: &str) {
        let passwd = String::from(passwd) + "_center";
        let hash = ring::digest::digest(&ring::digest::SHA256, passwd.as_bytes());
    }

    #[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
    #[repr(i32)]
    pub enum Code {
        Success = 0,
        SysError = 1,
        DBError = 2,
        LoginError = 3,
        AuthError = 4,
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
}
*/