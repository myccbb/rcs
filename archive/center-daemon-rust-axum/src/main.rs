mod dist;
mod deploy;
mod task;

use axum;

use tracing;
use tracing_subscriber;

use clap::Parser;


#[tokio::main]
async fn main() -> Result<(), axum::Error> {
    tracing_subscriber::fmt::init();

    let args: dist::Args = dist::Args::parse();
    tracing::info!("args {:?}", args);

    let cfg_str = std::fs::read_to_string(args.conf).unwrap();
    let cfg = dist::Config::new(cfg_str.as_str()).unwrap();

    tracing::warn!(hello="hello");

    let (tx, rx) = tokio::sync::mpsc::channel(1024);

    let deploy_config = cfg.deploy.clone();
    tokio::spawn(async move { task::task_handler(rx).await });

    let app = axum::Router::new()
        .route("/center-daemon/api/v1/deploy",
               axum::routing::get(deploy::deploy))
        .layer(axum::Extension(cfg.clone()))
        .layer(axum::Extension(tx));

    axum::Server::bind(&format!("{}:{}", cfg.api.host, cfg.api.port).parse().unwrap())
        .serve(app.into_make_service())
        .await.unwrap();
    Ok(())
}
