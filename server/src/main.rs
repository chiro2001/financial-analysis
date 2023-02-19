use anyhow::Result;

use tonic::{Request, Response, Status};
use tonic::transport::Server;
use tracing::info;
use rpc::api::api_rpc_server::{ApiRpc, ApiRpcServer};
use rpc::api::RegisterRequest;

#[derive(Default)]
pub struct ApiServer {}

#[tonic::async_trait]
impl ApiRpc for ApiServer {
    async fn register(&self, request: Request<RegisterRequest>) -> Result<Response<()>, Status> {
        let data = request.into_inner();
        info!("req: {}, {}", data.username, data.password);
        Ok(Response::new(()))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "[::1]:51411".parse().unwrap();
    let greeter = ApiServer::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(ApiRpcServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}