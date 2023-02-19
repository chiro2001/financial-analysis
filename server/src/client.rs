use anyhow::Result;
use rpc::api::LoginRegisterRequest;
use rpc::API_PORT;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let addr = format!("http://127.0.0.1:{}", API_PORT);

    let mut client = rpc::api::api_rpc_client::ApiRpcClient::connect(addr.clone()).await?;
    info!("got client: {:?}", client);
    let r = client.login(LoginRegisterRequest::default()).await?;
    info!("login resp: {:?}", r);

    let mut client = rpc::api::register_client::RegisterClient::connect(addr.clone()).await?;
    info!("got register client: {:?}", client);
    let r = client.register(LoginRegisterRequest::default()).await?;
    info!("register resp: {:?}", r);

    Ok(())
}