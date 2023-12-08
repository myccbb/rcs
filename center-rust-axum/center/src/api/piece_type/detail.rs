use axum::extract;

use serde::{Deserialize, Serialize};

use tracing as log;

use crate::db::PieceType;
use crate::dist::api::{response, success, Code, Res, ServerContext};

#[derive(Serialize, Deserialize)]
pub struct DetailReq {
    id: String,
}

#[axum_macros::debug_handler]
pub async fn detail(
    server_context: extract::Extension<ServerContext>,
    req: extract::Query<DetailReq>,
) -> axum::Json<Res<Option<PieceType>>> {
    let conn = &server_context.center_conn;
    let piece_type = PieceType::get_by_id(conn, req.id.as_str()).await;
    if let Err(e) = piece_type {
        log::info!("failed to list piece type, {:?}", e);
        return axum::Json(response(
            Code::SysError,
            "failed to list piece type".into(),
            None,
        ));
    }
    let piece_type = piece_type.unwrap();
    axum::Json(success(piece_type))
}
