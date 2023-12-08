use axum;
use tokio;
use serde::{Serialize, Deserialize};
use crate::task;
use crate::dist::api;
use crate::dist;


pub async fn deploy(
    headers: axum::headers::HeaderMap,
    tx: axum::Extension<tokio::sync::mpsc::Sender<task::TaskInfo>>,
    cfg: axum::Extension<dist::Config>,
    req: axum::extract::Query<std::collections::HashMap<String, String>>,
) -> axum::Json<api::Res<Option<task::TaskInfo>>> {
    let project_name = req.get("project_name");
    if let None = project_name {
        return axum::Json(api::response(
            api::Code::InvalidParam,
            "project_name is required".into(),
            None));
    }
    let project_name = project_name.unwrap();
    if headers.get("token").is_none() {
        return axum::Json(api::response(
            api::Code::InvalidParam,
            "token header is required".into(),
            None));
    }
    let token = headers.get("token").unwrap();
    if token.to_str().is_err() {
        return axum::Json(api::response(
            api::Code::InvalidParam,
            "token header is invalid".into(),
            None));
    }
    let token = token.to_str().unwrap();
    if token != cfg.deploy.token {
        return axum::Json(api::response(
            api::Code::AuthError,
            "token is invalid".into(),
            None));
    }
    let project_config = cfg.deploy.get_project_config(project_name);
    if let None = project_config {
        return axum::Json(api::response(
            api::Code::InvalidParam,
            "project_name is invalid".into(),
            None));
    }
    let task_info = task::TaskInfo {
        task_type: task::TaskType::Deploy,
        create_time: "2021-01-01 00:00:00".into(),
        project_base: cfg.deploy.project_base.clone(),
        project_config: project_config.unwrap(),
    };

    let result = tx.send(task_info.clone()).await;
    if let Err(e) = result {
        return axum::Json(api::response(
            api::Code::InternalError,
            format!("send task failed: {}", e),
            None));
    }
    axum::Json(api::success(Some(task_info)))
}