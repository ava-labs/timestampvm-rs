//! Implements client for timestampvm API.

use std::{
    collections::HashMap,
    io::{self, Error, ErrorKind},
};

use avalanche_types::{ids, jsonrpc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PingResponse {
    pub jsonrpc: String,
    pub id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<crate::api::PingResponse>,
}

pub async fn ping(http_rpc: &str, url_path: &str) -> io::Result<PingResponse> {
    log::info!("ping {http_rpc} with {url_path}");

    let mut data = jsonrpc::RequestWithParamsArray::default();
    data.method = String::from("timestampvm.ping");

    let d = data.encode_json()?;
    let rb = http_manager::post_non_tls(http_rpc, url_path, &d).await?;

    serde_json::from_slice(&rb)
        .map_err(|e| Error::new(ErrorKind::Other, format!("failed ping '{}'", e)))
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LastAcceptedResponse {
    pub jsonrpc: String,
    pub id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<crate::api::chain_handlers::LastAcceptedResponse>,
}

pub async fn last_accepted(http_rpc: &str, url_path: &str) -> io::Result<LastAcceptedResponse> {
    log::info!("last_accepted {http_rpc} with {url_path}");

    let mut data = jsonrpc::RequestWithParamsArray::default();
    data.method = String::from("timestampvm.lastAccepted");

    let d = data.encode_json()?;
    let rb = http_manager::post_non_tls(http_rpc, url_path, &d).await?;

    serde_json::from_slice(&rb)
        .map_err(|e| Error::new(ErrorKind::Other, format!("failed last_accepted '{}'", e)))
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetBlockResponse {
    pub jsonrpc: String,
    pub id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<crate::api::chain_handlers::GetBlockResponse>,
}

pub async fn get_block(
    http_rpc: &str,
    url_path: &str,
    id: &ids::Id,
) -> io::Result<GetBlockResponse> {
    log::info!("get_block {http_rpc} with {url_path}");

    let mut data = jsonrpc::RequestWithParamsHashMapArray::default();
    data.method = String::from("timestampvm.getBlock");

    let mut m = HashMap::new();
    m.insert("id".to_string(), id.to_string());

    let params = vec![m];
    data.params = Some(params);

    let d = data.encode_json()?;
    let rb = http_manager::post_non_tls(http_rpc, url_path, &d).await?;

    serde_json::from_slice(&rb)
        .map_err(|e| Error::new(ErrorKind::Other, format!("failed get_block '{}'", e)))
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProposeBlockResponse {
    pub jsonrpc: String,
    pub id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<crate::api::chain_handlers::ProposeBlockResponse>,
}

pub async fn propose_block(
    http_rpc: &str,
    url_path: &str,
    d: Vec<u8>,
) -> io::Result<ProposeBlockResponse> {
    log::info!("propose_block {http_rpc} with {url_path}");

    let mut data = jsonrpc::RequestWithParamsHashMapArray::default();
    data.method = String::from("timestampvm.proposeBlock");

    let mut m = HashMap::new();
    m.insert("data".to_string(), base64::encode(&d));

    let params = vec![m];
    data.params = Some(params);

    let d = data.encode_json()?;
    let rb = http_manager::post_non_tls(http_rpc, url_path, &d).await?;

    serde_json::from_slice(&rb)
        .map_err(|e| Error::new(ErrorKind::Other, format!("failed propose_block '{}'", e)))
}
