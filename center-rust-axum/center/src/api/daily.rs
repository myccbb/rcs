use axum_macros;

use crate::dist::api::{response, success, Code, Res, ServerContext};

#[axum_macros::debug_handler]
pub async fn loading(
    server_context: axum::extract::Extension<ServerContext>,
    // raw_body: axum::body::Bytes,
) -> axum::Json<Res<Option<crate::service::Daily>>> {
    // let req = serde_json::from_slice::<CreateReq>(&raw_body);
    // if let Err(e) = req {
    //     log::error!("error: {}", e.to_string());
    //     return axum::Json(response(Code::InvalidParam, e.to_string(), None));
    // }
    // let req = req.unwrap();
    // if let Err(e) = req.is_valid() {
    //     return axum::Json(response(Code::InvalidParam, e.to_string(), None));
    // }
    let result = crate::service::Daily::new(&server_context.center_conn).await;
    if let Err(e) = result {
        log::error!("error: {}", e.to_string());
        return axum::Json(response(Code::SysError, e.to_string(), None));
    }
    axum::Json(success(result.unwrap()))
}

// #[axum_macros::debug_handler]
// pub async fn create_matter_collection(
//     server_context: axum::extract::Extension<ServerContext>,
//     raw_body: axum::body::Bytes,
// ) -> axum::Json<Res<Option<crate::service::MatterCollectionRef>>> {
//     let daily = crate::service::Daily::new(&server_context.center_pool).await;
//     axum::Json(success(None))
// }
