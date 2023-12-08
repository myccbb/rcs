#[derive(clap::Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long, default_value = "config.json")]
    pub conf: String,
}

use serde::{self, Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Clone)]
pub struct ApiConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DeployConfig {
    pub token: String,
    pub project_base: String,
    pub project_list: Vec<ProjectConfig>,

    #[serde(skip_serializing, skip_deserializing)]
    project_map: std::collections::HashMap<String, ProjectConfig>,
}

impl DeployConfig {
    pub fn init(&mut self)  {
        for project in &self.project_list {
            self.project_map.insert(project.project_name.clone(), project.clone());
        }
    }

    pub fn get_project_config(&self, project_name: &str) -> Option<ProjectConfig> {
        self.project_map.get(project_name).cloned()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectConfig {
    pub project_name: String,
    pub deploy_command: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub api: ApiConfig,
    pub deploy: DeployConfig,
}

impl Config {
    pub fn new(cfg_str: &str) -> Result<Self, serde_json::Error> {
        let mut cfg = serde_json::from_str::<Config>(cfg_str)?;
        cfg.deploy.init();
        Ok(cfg)
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
        Res {
            code,
            msg,
            data,
        }
    }


    #[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
    #[repr(i32)]
    pub enum Code {
        Success = 0,
        InternalError = 1,
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
