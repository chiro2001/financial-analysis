#![allow(non_snake_case)]

use std::time::Duration;
use anyhow::Result;
use tower_http::cors::{AllowOrigin, CorsLayer};
use http::header::HeaderName;
use tonic::{Code, Request, Response, Status};
use tonic::transport::Server;
use tracing::info;
use rpc::api::api_rpc_server::ApiRpc;
use rpc::api::{CashFlow, DebtDecapitalStructure, GuideLineRequest, GuideLineResp, IncomeAnalysisRequest, IncomeAnalysisResp, LoginRegisterRequest, LoginResp, OperationAbility, PredictRequest, PredictResp, Profitability, ReasonResp, ShareIndex, StockIssueRequest, StockIssueResp, StockListResp, StockResp, TradingHistoryItem, TradingHistoryRequest, TradingHistoryResp};
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

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct StockIssueResp2 {
    pub market: String,
    pub consignee: String,
    pub underwriting: String,
    pub sponsor: String,
    pub issue_price: String,
    pub issue_mode: String,
    pub issue_pe: String,
    pub pre_capital: String,
    pub capital: String,
    pub issue_volume: String,
    pub expected_fundraising: String,
    pub fundraising: String,
    pub issue_cost: String,
    pub net_amount_raised: String,
    pub underwriting_fee: String,
    pub announcement_date: String,
    pub launch_date: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct StockIssueRespWrapper {
    pub stock_issue: Vec<StockIssueResp2>,
}

impl Into<StockIssueResp> for StockIssueResp2 {
    fn into(self) -> StockIssueResp {
        StockIssueResp {
            market: self.market,
            consignee: self.consignee,
            underwriting: self.underwriting,
            sponsor: self.sponsor,
            issue_price: self.issue_price,
            issue_mode: self.issue_mode,
            issue_pe: self.issue_pe,
            pre_capital: self.pre_capital,
            capital: self.capital,
            issue_volume: self.issue_volume,
            expected_fundraising: self.expected_fundraising,
            fundraising: self.fundraising,
            issue_cost: self.issue_cost,
            net_amount_raised: self.net_amount_raised,
            underwriting_fee: self.underwriting_fee,
            announcement_date: self.announcement_date,
            launch_date: self.launch_date,
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, Default)]
pub struct GuideLineResp2 {
    pub share_index: Vec<ShareIndex2>,
    pub profitability: Vec<Profitability2>,
    pub operation_ability: Vec<OperationAbility2>,
    pub debt_decapital_structure: Vec<DebtDecapitalStructure2>,
    pub cash_flow: Vec<CashFlow2>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, Default)]
pub struct ShareIndex2 {
    pub date: String,
    pub Diluted_EPS: String,
    pub EPSWA: String,
    pub AEPS: String,
    pub EPS_NGOL: String,
    pub BPS: String,
    pub BPS_Adjusted: String,
    pub OCFPS: String,
    pub CRPS: String,
    pub UDPPS: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, Default)]
pub struct Profitability2 {
    pub Date: String,
    pub OROA: String,
    pub OPE: String,
    pub PROA: String,
    pub ROPTC: String,
    pub OPR: String,
    pub COGSTS: String,
    pub PMOS: String,
    pub DOE: String,
    pub ROC: String,
    pub ROA: String,
    pub SGPR: String,
    pub POTE: String,
    pub NMP: String,
    pub POMP: String,
    pub RR: String,
    pub ROI: String,
    pub GP: String,
    pub ROE: String,
    pub ROEWA: String,
    pub NPAD: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, Default)]
pub struct OperationAbility2 {
    pub Date: String,
    pub ART: String,
    pub DSO: String,
    pub DSI: String,
    pub RST: String,
    pub TFA: String,
    pub TATO: String,
    pub TATD: String,
    pub CATA: String,
    pub DCAT: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, Default)]
pub struct DebtDecapitalStructure2 {
    pub Date: String,
    pub AR: String,
    pub QR: String,
    pub CR: String,
    pub ICR: String,
    pub LDWCR: String,
    pub EAR: String,
    pub LDR: String,
    pub REFA: String,
    pub DER: String,
    pub RLALF: String,
    pub MCR: String,
    pub FANWR: String,
    pub CIR: String,
    pub ER: String,
    pub LVR: String,
    pub POFA: String,
    pub LEV: String,
    pub ASSET: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, Default)]
pub struct CashFlow2 {
    pub Date: String,
    pub NOCFTSR: String,
    pub ROOCFOA: String,
    pub NOCFTNP: String,
    pub NOCFTDR: String,
    pub CFR: String,
}

impl Into<GuideLineResp> for GuideLineResp2 {
    fn into(self) -> GuideLineResp {
        GuideLineResp {
            share_index: self.share_index.into_iter().map(|x| ShareIndex {
                date: x.date,
                diluted_eps: x.Diluted_EPS,
                epswa: x.EPSWA,
                aeps: x.AEPS,
                eps_ngol: x.EPS_NGOL,
                bps: x.BPS,
                bps_adjusted: x.BPS_Adjusted,
                ocfps: x.OCFPS,
                crps: x.CRPS,
                udpps: x.UDPPS,
            }).collect(),
            profitability: self.profitability.into_iter().map(|x| Profitability {
                date: x.Date,
                oroa: x.OROA,
                ope: x.OPE,
                proa: x.PROA,
                roptc: x.ROPTC,
                opr: x.OPR,
                cogsts: x.COGSTS,
                pmos: x.PMOS,
                doe: x.DOE,
                roc: x.ROC,
                roa: x.ROA,
                sgpr: x.SGPR,
                pote: x.POTE,
                nmp: x.NMP,
                pomp: x.POMP,
                rr: x.RR,
                roi: x.ROI,
                gp: x.GP,
                roe: x.ROE,
                roewa: x.ROEWA,
                npad: x.NPAD,
            }).collect(),
            operation_ability: self.operation_ability.into_iter().map(|x| OperationAbility {
                date: x.Date,
                art: x.ART,
                dso: x.DSO,
                dsi: x.DSI,
                rst: x.RST,
                tfa: x.TFA,
                tato: x.TATO,
                tatd: x.TATD,
                cata: x.CATA,
                dcat: x.DCAT,
            }).collect(),
            debt_decapital_structure: self.debt_decapital_structure.into_iter().map(|x| DebtDecapitalStructure {
                date: x.Date,
                ar: x.AR,
                qr: x.QR,
                cr: x.CR,
                icr: x.ICR,
                ldwcr: x.LDWCR,
                ear: x.EAR,
                ldr: x.LDR,
                refa: x.REFA,
                der: x.DER,
                rlalf: x.RLALF,
                mcr: x.MCR,
                fanwr: x.FANWR,
                cir: x.CIR,
                er: x.ER,
                lvr: x.LVR,
                pofa: x.POFA,
                lev: x.LEV,
                asset: x.ASSET,
            }).collect(),
            cash_flow: self.cash_flow.into_iter().map(|x| CashFlow {
                date: x.Date,
                nocftsr: x.NOCFTSR,
                roocfoa: x.ROOCFOA,
                nocftnp: x.NOCFTNP,
                nocftdr: x.NOCFTDR,
                cfr: x.CFR,
            }).collect(),
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct PredictResp2 {
    pub result: Vec<f32>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct PredictRequest2 {
    pub data: Vec<f32>,
    pub length: u32,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, Default)]
pub struct IncomeAnalysisResp2 {
    pub incomes: Vec<f32>,
    pub ave: f32,
}

impl Into<IncomeAnalysisResp> for IncomeAnalysisResp2 {
    fn into(self) -> IncomeAnalysisResp {
        IncomeAnalysisResp { incomes: self.incomes, ave: self.ave }
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
            /*TradingHistoryType::Week*/ 1 => "Week",
            /*TradingHistoryType::Month*/ 2 => "Month",
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

    async fn predict_data(&self, request: Request<PredictRequest>) -> std::result::Result<Response<PredictResp>, Status> {
        let req = request.into_inner();
        let url = format!("{}/data_predict", JRPC_HTTP_PREFIX);
        let client = reqwest::Client::new();
        let resp = client.post(url)
            .json(&PredictRequest2 { data: req.data, length: req.length })
            .send().await.map_err(|e| Status::unknown(format!("Internal Error: {}", e)))?
            .json::<JrpcResp<PredictResp2>>()
            .await.map_err(|e| Status::new(Code::Aborted, format!("Decode Error: {}", e)))?;
        if resp.code != 200 {
            Err(Status::unknown(format!("Internal Error: {:?}", resp)))
        } else {
            Ok(Response::new(PredictResp { data: resp.data.result }))
        }
    }

    async fn stock_issue(&self, request: Request<StockIssueRequest>) -> std::result::Result<Response<StockIssueResp>, Status> {
        let data = request.into_inner();
        let url = format!("{}/symbols/getStockIssue?a={}", JRPC_HTTP_PREFIX, data.symbol);
        info!("requesting url: {}", url);
        let resp = reqwest::get(url)
            .await.map_err(|e| Status::new(Code::Aborted, format!("Network Error: {}", e)))?
            .text().await.map_err(|e| Status::unknown(format!("Network Error: {}", e)))?
            .replace("p/e", "pe");
        let resp: JrpcResp<StockIssueRespWrapper> = serde_json::from_str(resp.as_str()).map_err(|e| Status::new(Code::Aborted, format!("Decode Error: {}", e)))?;
        if resp.code != 200 || resp.data.stock_issue.is_empty() {
            Err(Status::unknown(format!("Internal Error: {:?}", resp)))
        } else {
            Ok(Response::new(resp.data.stock_issue.first().unwrap().clone().into()))
        }
    }

    async fn guide_line(&self, request: Request<GuideLineRequest>) -> std::result::Result<Response<GuideLineResp>, Status> {
        let data = request.into_inner();
        let url = format!("{}/finance/getGuideLine?a={}&b={}", JRPC_HTTP_PREFIX, data.code, data.year);
        info!("requesting url: {}", url);
        let resp = reqwest::get(url)
            .await.map_err(|e| Status::new(Code::Aborted, format!("Network Error: {}", e)))?
            .json::<JrpcResp<GuideLineResp2>>()
            .await.map_err(|e| Status::new(Code::Aborted, format!("Decode Error: {}", e)))?;
        if resp.code != 200 {
            Err(Status::unknown(format!("Internal Error: {:?}", resp)))
        } else {
            let data = resp.data.into();
            info!("guide line done {:?}", data);
            Ok(Response::new(data))
        }
    }

    async fn income_analysis(&self, request: Request<IncomeAnalysisRequest>) -> std::result::Result<Response<IncomeAnalysisResp>, Status> {
        let data = request.into_inner();
        let url = format!("{}/income_analysis/{}", JRPC_HTTP_PREFIX, data.code);
        info!("requesting url: {}", url);
        let resp = reqwest::get(url)
            .await.map_err(|e| Status::new(Code::Aborted, format!("Network Error: {}", e)))?
            .json::<JrpcResp<IncomeAnalysisResp2>>()
            .await.map_err(|e| Status::new(Code::Aborted, format!("Decode Error: {}", e)))?;
        if resp.code != 200 {
            Err(Status::unknown(format!("Internal Error: {:?}", resp)))
        } else {
            Ok(Response::new(resp.data.into()))
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