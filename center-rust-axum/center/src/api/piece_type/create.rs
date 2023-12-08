use axum::{self, extract};

use serde::{Deserialize, Serialize};

use tracing as log;

use crate::db::PieceType;
use crate::dist::api::{response, success, Code, Res, ServerContext};
use crate::dist::utils;

use crate::db::Error as DBError;

#[derive(Serialize, Deserialize)]
pub struct CreateReq {
    id: String,
    name: String,
    #[serde(default)]
    desc: String,
}

#[axum_macros::debug_handler]
pub async fn create(
    server_context: extract::Extension<ServerContext>,
    raw_body: axum::body::Bytes,
) -> axum::Json<Res<Option<()>>> {
    let req = serde_json::from_slice::<CreateReq>(&raw_body);
    if let Err(e) = req {
        log::error!("error: {}", e.to_string());
        return axum::Json(response(Code::InvalidParam, e.to_string(), None));
    }
    let req = req.unwrap();
    if let Err(e) = req.is_valid() {
        return axum::Json(response(Code::InvalidParam, e.to_string(), None));
    }
    let now_utc = utils::now_beijing();
    let mut piece_type = PieceType {
        internal_id: 0,
        id: req.id.clone(),
        name: req.name.clone(),
        description: req.desc.clone(),
        create_time: now_utc,
        update_time: now_utc,
    };
    let result = piece_type.create_sync(&server_context.center_conn);
    if let Err(e) = result {
        log::info!(
            "failed to create piece type, {}, {}",
            piece_type.to_short_string(),
            e,
        );
        if e == DBError::DuplicateEntry {
            return axum::Json(response(Code::ObjectExist, "piece exist".into(), None));
        }
        return axum::Json(response(
            Code::DBError,
            "failed to create piece type".into(),
            None,
        ));
    }
    axum::Json(success(None))
}

impl CreateReq {
    fn is_valid(&self) -> Result<bool, Error> {
        if self.id.is_empty() {
            return Err(Error::InvalidId);
        }
        if self.name.is_empty() {
            return Err(Error::InvalidName);
        }
        Ok(true)
    }
}

enum Error {
    InvalidId,
    InvalidName,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidId => write!(f, "invalid id"),
            Self::InvalidName => write!(f, "invalid name"),
        }
    }
}
