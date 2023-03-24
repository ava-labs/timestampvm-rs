//! Implements static handlers specific to this VM.
//! To be served via `[HOST]/ext/vm/[VM ID]/static`.

use std::io;

use avalanche_types::{proto::http::Element, subnet::rpc::http::handle::Handle};
use bytes::Bytes;
use jsonrpc_core::{BoxFuture, IoHandler, MethodCall, Result};
use jsonrpc_derive::rpc;

/// Defines static handler RPCs for this VM.
#[rpc]
pub trait Rpc {
    #[rpc(name = "ping", alias("timestampvm.ping"))]
    fn ping(&self) -> BoxFuture<Result<crate::api::PingResponse>>;
}

/// Implements API services for the static handlers.
#[derive(Default)]
pub struct StaticService {}

impl StaticService {
    pub fn new() -> Self {
        Self {}
    }
}

impl Rpc for StaticService {
    fn ping(&self) -> BoxFuture<Result<crate::api::PingResponse>> {
        log::debug!("ping called");
        Box::pin(async move { Ok(crate::api::PingResponse { success: true }) })
    }
}
#[derive(Clone)]
pub struct StaticHandler {
    pub handler: IoHandler,
}

impl StaticHandler {
    pub fn new(service: StaticService) -> Self {
        let mut handler = jsonrpc_core::IoHandler::new();
        handler.extend_with(Rpc::to_delegate(service));
        Self { handler }
    }
}

#[tonic::async_trait]
impl Handle for StaticHandler {
    async fn request(
        &self,
        req: &Bytes,
        _headers: &Vec<Element>,
    ) -> std::io::Result<(Bytes, Vec<Element>)> {
       let de_request: MethodCall = serde_json::from_slice(req).map_err(|e| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("failed to deserialize request: {e}"),
            )
        })?;

        let json_str = serde_json::to_string(&de_request).map_err(|e| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("failed to serialize request: {e}"),
            )
        })?;

        match self.handler.handle_request(&json_str).await {
            Some(resp) => Ok((Bytes::from(resp), Vec::new())),
            None => Err(io::Error::new(
                io::ErrorKind::Other,
                "failed to handle request",
            )),
        }
    }
}
