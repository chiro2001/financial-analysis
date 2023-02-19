use anyhow::Result;

use tonic::{Request, Response, Status};
use tonic::transport::Server;
use tracing::info;
use rpc::api::api_rpc_server::{ApiRpc, ApiRpcServer};
use rpc::api::{LoginRegisterRequest, ReasonResp};
use rpc::api::register_server::Register;
use rpc::API_PORT;

#[derive(Default)]
pub struct ApiServer {}

#[tonic::async_trait]
impl ApiRpc for ApiServer {
    async fn ping(&self, _request: Request<()>) -> std::result::Result<Response<()>, Status> {
        info!("ping");
        Ok(Response::new(()))
    }

    async fn login(&self, request: Request<LoginRegisterRequest>) -> std::result::Result<Response<ReasonResp>, Status> {
        let data = request.into_inner();
        info!("login: {:?}", data);
        Ok(Response::new(ReasonResp::default()))
    }
}

#[tonic::async_trait]
impl Register for ApiServer {
    async fn register(&self, request: Request<LoginRegisterRequest>) -> std::result::Result<Response<ReasonResp>, Status> {
        let data = request.into_inner();
        info!("register: {:?}", data);
        Ok(Response::new(ReasonResp::default()))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    // let addr = format!("[::1]:{}", API_PORT).parse().unwrap();
    let addr = format!("0.0.0.0:{}", API_PORT).parse().unwrap();
    let api = ApiServer::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(ApiRpcServer::new(api))
        .serve(addr)
        .await?;

    Ok(())
}