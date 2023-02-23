use anyhow::Result;
use rpc::api::LoginRegisterRequest;
use rpc::API_PORT;
use tracing::info;

// 对 gRPC 两个 Service 的基本连接测试
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let addr = format!("http://0.0.0.0:{}", API_PORT);

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