use crate::vm;
use jsonrpc_core::{BoxFuture, Result};
use jsonrpc_derive::rpc;
use serde::{Deserialize, Serialize};

#[rpc]
pub trait Rpc {
    #[rpc(name = "ping")]
    fn ping(&self) -> BoxFuture<Result<PingResponse>>;
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PingResponse {
    pub success: bool,
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
    fn ping(&self) -> BoxFuture<Result<PingResponse>> {
        log::debug!("ping called");
        Box::pin(async move { Ok(PingResponse { success: true }) })
    }
}
