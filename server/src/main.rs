use std::time::Duration;
use anyhow::Result;
use tower_http::cors::{AllowOrigin, CorsLayer};
use http::header::HeaderName;
use tonic::{Code, Request, Response, Status};
use tonic::transport::Server;
use tracing::info;
use rpc::api::api_rpc_server::ApiRpc;
use rpc::api::{LoginRegisterRequest, LoginResp, ReasonResp, StockListResp, StockResp, TradingHistoryItem, TradingHistoryRequest, TradingHistoryResp};
use rpc::api::register_server::{Register, RegisterServer};
use rpc::API_PORT;
use tonic_web::GrpcWebLayer;

pub const JRPC_HTTP_PREFIX: &'static str = "http://127.0.0.1:8000/api/v1";

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct JrpcResp<T> {
    pub code: usize,
    pub message: String,
    pub data: T,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct StockResp2 {
    pub symbol: String,
    pub code: String,
    pub name: String,
    pub _id: String,
}

impl Into<StockResp> for StockResp2 {
    fn into(self) -> StockResp {
        StockResp {
            symbol: self.symbol,
            code: self.code,
            name: self.name,
            id: self._id,
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct TradingHistoryItem2 {
    pub date: String,
    pub open: String,
    pub close: String,
    pub high: String,
    pub low: String,
    pub volume: String,
}

impl Into<TradingHistoryItem> for TradingHistoryItem2 {
    fn into(self) -> TradingHistoryItem {
        TradingHistoryItem {
            date: self.date,
            open: self.open,
            close: self.close,
            high: self.high,
            low: self.low,
            volume: self.volume,
        }
    }
}

#[derive(Default)]
pub struct ApiServer {}

#[tonic::async_trait]
impl ApiRpc for ApiServer {
    async fn ping(&self, _request: Request<()>) -> std::result::Result<Response<()>, Status> {
        info!("ping");
        Ok(Response::new(()))
    }

    async fn login(&self, request: Request<LoginRegisterRequest>) -> std::result::Result<Response<LoginResp>, Status> {
        let data = request.into_inner();
        info!("login: {:?}", data);
        Ok(Response::new(LoginResp {
            err: false,
            token: "token".to_string(),
            reason: "".to_string(),
        }))
    }

    async fn stock_list(&self, _request: Request<()>) -> std::result::Result<Response<StockListResp>, Status> {
        // std::thread::sleep(std::time::Duration::from_secs(3));
        let resp = reqwest::get(format!("{}/stockList", JRPC_HTTP_PREFIX))
            .await.map_err(|e| Status::new(Code::Aborted, format!("Network Error: {}", e)))?
            .json::<JrpcResp<Vec<StockResp2>>>()
            .await.map_err(|e| Status::new(Code::Aborted, format!("Decode Error: {}", e)))?;
        if resp.code != 200 {
            return Err(Status::unknown(format!("Internal Error: {:?}", resp)));
        }
        Ok(Response::new(StockListResp { data: resp.data.into_iter().map(|x| x.into()).collect() }))
    }

    async fn trading_history(&self, request: Request<TradingHistoryRequest>) -> std::result::Result<Response<TradingHistoryResp>, Status> {
        let data = request.into_inner();
        let typ = match data.typ {
            /*TradingHistoryType::Daily*/ 0 => "Daily",
            /*TradingHistoryType::Month*/ 1 => "Month",
            /*TradingHistoryType::Week*/ 2 => "Week",
            _ => "Error"
        };
        let function = format!("get{}His", typ);
        let url = format!("{}/trading/{}?a={}", JRPC_HTTP_PREFIX, function, data.symbol);
        info!("requesting url: {}", url);
        let resp = reqwest::get(url)
            .await.map_err(|e| Status::new(Code::Aborted, format!("Network Error: {}", e)))?
            .json::<JrpcResp<Vec<TradingHistoryItem2>>>()
            .await.map_err(|e| Status::new(Code::Aborted, format!("Decode Error: {}", e)))?;
        if resp.code != 200 {
            Err(Status::unknown(format!("Internal Error: {:?}", resp)))
        } else {
            Ok(Response::new(TradingHistoryResp { data: resp.data.into_iter().map(|x| x.into()).collect() }))
        }
    }
}

#[derive(Default)]
pub struct RegisterService {}

#[tonic::async_trait]
impl Register for RegisterService {
    async fn register(&self, request: Request<LoginRegisterRequest>) -> std::result::Result<Response<ReasonResp>, Status> {
        let data = request.into_inner();
        info!("register: {:?}", data);
        Ok(Response::new(ReasonResp::default()))
    }
}

const DEFAULT_MAX_AGE: Duration = Duration::from_secs(24 * 60 * 60);
const DEFAULT_EXPOSED_HEADERS: [&str; 3] =
    ["grpc-status", "grpc-message", "grpc-status-details-bin"];
const DEFAULT_ALLOW_HEADERS: [&str; 5] =
    ["x-grpc-web", "content-type", "x-user-agent", "grpc-timeout", "authorization"];

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    // let addr = format!("[::1]:{}", API_PORT).parse().unwrap();
    let addr = format!("0.0.0.0:{}", API_PORT).parse().unwrap();
    let api = ApiServer::default();
    let check = |req: Request<()>| {
        let token = "token";
        info!("metadata: {:?}", req.metadata());
        match req.metadata().get("authorization") {
            Some(t) if token == t => Ok(req),
            Some(t) => Err(Status::unauthenticated(format!("No valid token: {:?}", t.to_str()))),
            _ => Err(Status::unauthenticated("No valid token")),
        }
    };
    let svc = rpc::api::api_rpc_server::ApiRpcServer::with_interceptor(api, check);
    let register_service = RegisterService::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .accept_http1(true)
        .layer(
            CorsLayer::new()
                .allow_origin(AllowOrigin::mirror_request())
                .allow_credentials(true)
                .max_age(DEFAULT_MAX_AGE)
                .expose_headers(
                    DEFAULT_EXPOSED_HEADERS
                        .iter()
                        .cloned()
                        .map(HeaderName::from_static)
                        .collect::<Vec<HeaderName>>(),
                )
                .allow_headers(
                    DEFAULT_ALLOW_HEADERS
                        .iter()
                        .cloned()
                        .map(HeaderName::from_static)
                        .collect::<Vec<HeaderName>>(),
                ),
        )
        .layer(GrpcWebLayer::new())
        .add_service(svc)
        .add_service(RegisterServer::new(register_service))
        .serve(addr)
        .await?;

    Ok(())
}