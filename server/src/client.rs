use anyhow::Result;
use rpc::api::LoginRegisterRequest;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let addr = "http://127.0.0.1:51411";
    let mut client = rpc::api::register_client::RegisterClient::connect(addr).await?;
    info!("got client: {:?}", client);
    let _r = client.register(LoginRegisterRequest::default()).await?;
    Ok(())
}