use anyhow::Result;
use rpc::api::RegisterRequest;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let addr = "http://127.0.0.1:51411";
    let mut client = rpc::api::api_rpc_client::ApiRpcClient::connect(addr).await?;
    info!("got client: {:?}", client);
    let _r = client.register(RegisterRequest::default()).await?;
    Ok(())
}