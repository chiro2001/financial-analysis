use anyhow::Result;
use rpc::api::LoginRegisterRequest;
use rpc::API_PORT;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let addr = format!("http://127.0.0.1:{}", API_PORT);
    let mut client = rpc::api::register_client::RegisterClient::connect(addr).await?;
    info!("got client: {:?}", client);
    let _r = client.register(LoginRegisterRequest::default()).await?;
    Ok(())
}