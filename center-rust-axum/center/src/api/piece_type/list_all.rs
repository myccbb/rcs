use axum::extract;

use serde::{Deserialize, Serialize};

use tracing as log;

use crate::db::PieceType;
use crate::dist::api::{response, success, Code, List, Res, ServerContext};

#[derive(Serialize, Deserialize)]
pub struct ListAllReq {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_page_size")]
    pub page_size: u32,
    #[serde(default)]
    pub name_like: String,
}
fn default_page() -> u32 {
    1
}
fn default_page_size() -> u32 {
    10
}

#[axum_macros::debug_handler]
pub async fn list_all(
    server_context: extract::Extension<ServerContext>,
    req: extract::Query<ListAllReq>,
) -> axum::Json<Res<Option<List<PieceType>>>> {
    let conn = &server_context.center_conn;
    let piece_type_list = PieceType::list_all(conn, req.page, req.page_size);
    if let Err(e) = piece_type_list {
        log::info!("failed to list piece type, {:?}", e);
        return axum::Json(response(
            Code::SysError,
            "failed to list piece type".into(),
            None,
        ));
    }
    let piece_type_list = piece_type_list.unwrap();
    let result = List {
        page: 0,
        page_size: piece_type_list.len(),
        total: 0,
        results: piece_type_list,
    };
    axum::Json(success(Some(result)))
}
