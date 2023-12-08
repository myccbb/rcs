use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use crate::dist;

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum TaskType {
    Deploy = 1,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskInfo {
    pub task_type: TaskType,
    pub create_time: String,

    pub project_base: String,

    pub project_config: dist::ProjectConfig,
}


pub async fn task_handler(
    mut rx: tokio::sync::mpsc::Receiver<TaskInfo>,
) {
    tracing::info!("task start");
    loop {
        let task = rx.recv().await;
        tracing::info!("get task");
        if let None = task {
            tracing::info!("skip empty task");
            continue;
        }
        let task = task.unwrap();
        match task.task_type {
            TaskType::Deploy => {
                _ = deploy_task(task);
            }
        }
    }
}


pub fn deploy_task(
    task_info: TaskInfo,
) -> Result<(), Box<dyn std::error::Error>> {
    let project_config = task_info.project_config;

    let project_dir = format!("{}/{}", task_info.project_base, project_config.project_name);
    for v in project_config.deploy_command {
        tracing::info!("executing command: {}", v.as_str());
        let result = std::process::Command::new("/usr/bin/bash")
            .current_dir(&project_dir)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .args(&["-c", v.as_str()])
            .output();
        if let Err(e) = result {
            tracing::error!("execute command failed: {}, {}", v.as_str(), e);
            return Err(Box::new(e));
        }
        let result = result.unwrap();
        if !result.status.success() {
            tracing::error!("execute command failed: {}", v.as_str());
            break;
        }
        tracing::info!("execute command finished: {}", result.status);
    }
    Ok(())
}

