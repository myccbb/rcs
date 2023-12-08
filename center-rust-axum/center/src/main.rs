use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

use axum::body;
use http::Request;
use tracing::field::Empty as EmptyField;

use axum::routing;

use tracing::{self, info};

use clap::Parser;

mod api;
mod db;
mod dist;
mod service;

#[tokio::main]
async fn main() -> Result<(), axum::Error> {
    tracing_subscriber::fmt::init();

    let args = dist::cfg::Args::parse();
    info!("args {:?}", args);

    let cfg_str = std::fs::read_to_string(args.conf).unwrap();
    let cfg = dist::cfg::Config::new(cfg_str.as_str()).unwrap();

    info!("config {:?}", cfg);

    let center_conn = sqlite::Connection::open_file(&cfg.db.db_name).unwrap();
    let server_context = dist::api::ServerContext::new(center_conn.clone());

    db::init_tables_sync(&center_conn).unwrap();
    service::init_daily(&center_conn).unwrap();

    let trace = TraceLayer::new_for_http()
        .make_span_with(|request: &Request<body::Body>| {
            tracing::info!("{} {}", request.method(), request.uri().path());
            tracing::info_span!("http-request", method = EmptyField, url = EmptyField)
        })
        .on_request(|request: &Request<body::Body>, span: &tracing::Span| {
            span.record("method", &tracing::field::display(request.method()));
            span.record("url", &tracing::field::display(request.uri().path()));
            tracing::debug!("started {} {}", request.method(), request.uri().path())
        });

    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(Any)
        .allow_headers(Any);

    // let middleware_stack = ServiceBuilder::new().layer(trace).layer(cors);

    let app = axum::Router::new()
        .route(
            "/center-server/api/v1/piece-type/list",
            routing::get(api::piece_type::list_all),
        )
        .route(
            "/center-server/api/v1/piece-type",
            routing::get(api::piece_type::detail),
        )
        .route(
            "/center-server/api/v1/piece-type",
            routing::post(api::piece_type::create),
        )
        .route(
            "/center-server/api/v1/daily/loading",
            routing::post(api::daily::loading),
        )
        // .layer(middleware_stack)
        .layer(axum::Extension(cfg))
        .layer(axum::Extension(server_context));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:50001")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
