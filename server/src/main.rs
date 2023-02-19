use anyhow::Result;

use tonic::{Request, Response, Status};
use tonic::transport::Server;
use tracing::info;
use rpc::api::api_rpc_server::{ApiRpc, ApiRpcServer};
use rpc::api::{LoginRegisterRequest, ReasonResp};
use rpc::api::register_server::Register;

#[derive(Default)]
pub struct ApiServer {}

#[tonic::async_trait]
impl ApiRpc for ApiServer {
    async fn ping(&self, request: Request<()>) -> std::result::Result<Response<()>, Status> {
        Ok(Response::new(()))
    }

    async fn login(&self, request: Request<LoginRegisterRequest>) -> std::result::Result<Response<ReasonResp>, Status> {
        Ok(Response::new(ReasonResp::default()))
    }
}

#[tonic::async_trait]
impl Register for ApiServer {
    async fn register(&self, request: Request<LoginRegisterRequest>) -> std::result::Result<Response<ReasonResp>, Status> {
        Ok(Response::new(ReasonResp::default()))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    // let addr = "[::1]:51411".parse().unwrap();
    let addr = "0.0.0.0:51411".parse().unwrap();
    let greeter = ApiServer::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(ApiRpcServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}