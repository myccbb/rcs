use tonic::{transport::Server, Request, Response, Status};

use tracing::{self, info};
use tracing_subscriber;

use clap::Parser;

mod dist;

use center::greeter_server::{Greeter, GreeterServer};
use center::{HelloReq, HelloRes};

pub mod center {
    tonic::include_proto!("center");
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(&self, request: Request<HelloReq>) -> Result<Response<HelloRes>, Status> {
        info!("Got a request: {:?}", request);

        let reply = center::HelloRes {
            message: format!("Hello {}!", request.into_inner().name).into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let args = dist::cfg::Args::parse();
    info!("args {:?}", args);

    let addr = "127.0.0.1:50051".parse()?;
    let greeter = MyGreeter::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
