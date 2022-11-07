use crate::vm;
use jsonrpc_core::{BoxFuture, Result};
use jsonrpc_derive::rpc;

#[rpc]
pub trait Rpc {
    #[rpc(name = "ping")]
    fn ping(&self) -> BoxFuture<Result<crate::api::PingResponse>>;
}

pub struct Service {
    pub vm: vm::Vm,
}

impl Service {
    pub fn new(vm: vm::Vm) -> Self {
        Self { vm }
    }
}

impl Rpc for Service {
    fn ping(&self) -> BoxFuture<Result<crate::api::PingResponse>> {
        log::debug!("ping called");
        Box::pin(async move { Ok(crate::api::PingResponse { success: true }) })
    }
}
