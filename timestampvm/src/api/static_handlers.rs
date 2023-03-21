//! Implements static handlers specific to this VM.
//! To be served via `[HOST]/ext/vm/[VM ID]/static`.

use crate::vm;
use avalanche_types::subnet::rpc::snow::engine::common::appsender::AppSender;
use jsonrpc_core::{BoxFuture, Result};
use jsonrpc_derive::rpc;

/// Defines static handler RPCs for this VM.
#[rpc]
pub trait Rpc {
    #[rpc(name = "ping", alias("timestampvm.ping"))]
    fn ping(&self) -> BoxFuture<Result<crate::api::PingResponse>>;
}

/// Implements API services for the static handlers.
pub struct Service<A>
where
    A: AppSender + Send + Sync + Clone + 'static,
{
    pub vm: vm::Vm<A>,
}

impl<A> Service<A>
where
    A: AppSender + Send + Sync + Clone + 'static,
{
    pub fn new(vm: vm::Vm<A>) -> Self {
        Self { vm }
    }
}

impl<A> Rpc for Service<A>
where
   A: AppSender + Send + Sync + Clone + 'static,
{
    fn ping(&self) -> BoxFuture<Result<crate::api::PingResponse>> {
        log::debug!("ping called");
        Box::pin(async move { Ok(crate::api::PingResponse { success: true }) })
    }
}
