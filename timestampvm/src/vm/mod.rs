use std::{
    collections::{HashMap, VecDeque},
    io::{self, Error, ErrorKind},
    sync::Arc,
};

use crate::{api, block::Block, genesis::Genesis, state};
use avalanche_types::{choices, ids, subnet};
use chrono::{DateTime, Utc};
use semver::Version;
use tokio::sync::{mpsc::Sender, RwLock};

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Represents Vm-specific states.
/// Defined in a separate struct, for interior mutability in "Vm".
/// To be protected with "Arc" and "RwLock".
pub struct VmState {
    pub ctx: Option<subnet::rpc::context::Context>,
    pub version: Version,
    pub genesis: Genesis,

    /// Represents persistent Vm state.
    pub state: Option<state::State>,
    /// Currently preferred block Id.
    pub preferred: ids::Id,
    /// Channel to send messages to the snowman consensus engine.
    pub to_engine: Option<Sender<subnet::rpc::common::message::Message>>,
    /// Set "true" to indicate that the Vm has finished bootstrapping
    /// for the chain.
    pub bootstrapped: bool,
}

impl Default for VmState {
    fn default() -> Self {
        Self {
            ctx: None,
            version: Version::new(0, 0, 0),
            genesis: Genesis::default(),

            state: None,
            preferred: ids::Id::empty(),
            to_engine: None,
            bootstrapped: false,
        }
    }
}

/// Implements block.ChainVM interface.
#[derive(Clone)]
pub struct Vm {
    pub state: Arc<RwLock<VmState>>,
    pub app_sender: Option<Box<dyn subnet::rpc::common::appsender::AppSender + Send + Sync>>,

    /// A queue of data that have not been put into a block and proposed yet.
    /// Mempool is not persistent, so just keep in memory via Vm.
    pub mempool: Arc<RwLock<VecDeque<Vec<u8>>>>,
}

impl Vm {
    pub fn new() -> Box<dyn subnet::rpc::vm::Vm + Send + Sync> {
        Box::new(Vm {
            state: Arc::new(RwLock::new(VmState::default())),
            app_sender: None,
            mempool: Arc::new(RwLock::new(VecDeque::with_capacity(100))),
        })
    }

    pub async fn is_bootstrapped(&self) -> bool {
        let vm_state = self.state.read().await;
        vm_state.bootstrapped
    }

    /// Signal consensus engine that a new block is ready to be created.
    pub async fn notify_block_ready(&self) {
        let vm_state = self.state.read().await;
        if let Some(to_engine) = &vm_state.to_engine {
            if to_engine
                .send(subnet::rpc::common::message::Message::PendingTxs)
                .await
                .is_err()
            {
                log::warn!("dropping message to consensus engine");
            };
            return;
        }

        log::error!("consensus engine channel failed to initialized");
        return;
    }

    pub async fn propose_block(&self, d: Vec<u8>) {
        let mut mempool = self.mempool.write().await;
        mempool.push_back(d);
        self.notify_block_ready().await;
    }

    pub async fn set_state(&self, snow_state: subnet::rpc::snow::State) -> io::Result<()> {
        let mut vm_state = self.state.write().await;
        match snow_state.try_into() {
            // called by chains manager when it is creating the chain.
            Ok(subnet::rpc::snow::State::Initializing) => {
                log::info!("set_state: initializing");
                vm_state.bootstrapped = false;
                Ok(())
            }

            Ok(subnet::rpc::snow::State::StateSyncing) => {
                log::info!("set_state: state syncing");
                Err(Error::new(ErrorKind::Other, "state sync is not supported"))
            }

            // called by the bootstrapper to signal bootstrapping has started.
            Ok(subnet::rpc::snow::State::Bootstrapping) => {
                log::info!("set_state: bootstrapping");
                vm_state.bootstrapped = false;
                Ok(())
            }

            // called when consensus has started signalling bootstrap phase is complete.
            Ok(subnet::rpc::snow::State::NormalOp) => {
                log::info!("set_state: normal op");
                vm_state.bootstrapped = true;
                Ok(())
            }

            Err(_) => Err(Error::new(ErrorKind::Other, "unknown state")),
        }
    }

    pub async fn set_preference(&self, id: ids::Id) -> io::Result<()> {
        let mut vm_state = self.state.write().await;
        vm_state.preferred = id;

        Ok(())
    }

    pub async fn last_accepted(&self) -> io::Result<ids::Id> {
        let vm_state = self.state.read().await;
        if let Some(state) = &vm_state.state {
            let blk_id = state.get_last_accepted_block_id().await?;
            return Ok(blk_id);
        }
        return Err(Error::new(ErrorKind::NotFound, "state manager not found"));
    }
}

impl avalanche_types::rpcchainvm::vm::Vm for Vm {}

#[tonic::async_trait]
impl subnet::rpc::common::vm::Vm for Vm {
    async fn initialize(
        &mut self,
        ctx: Option<subnet::rpc::context::Context>,
        db_manager: Box<dyn subnet::rpc::database::manager::Manager + Send + Sync>,
        genesis_bytes: &[u8],
        _upgrade_bytes: &[u8],
        _config_bytes: &[u8],
        to_engine: Sender<subnet::rpc::common::message::Message>,
        _fxs: &[subnet::rpc::common::vm::Fx],
        app_sender: Box<dyn subnet::rpc::common::appsender::AppSender + Send + Sync>,
    ) -> io::Result<()> {
        let mut vm_state = self.state.write().await;

        vm_state.ctx = ctx;

        let version =
            Version::parse(VERSION).map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;
        vm_state.version = version;

        let genesis = Genesis::from_slice(genesis_bytes)?;
        vm_state.genesis = genesis;

        let current = db_manager.current().await?;
        let state = state::State {
            db: Arc::new(RwLock::new(current.db)),
        };
        vm_state.state = Some(state.clone());

        vm_state.to_engine = Some(to_engine);

        self.app_sender = Some(app_sender);

        let has_last_accepted = state.has_last_accepted_block().await?;
        if has_last_accepted {
            let last_accepted_blk_id = state.get_last_accepted_block_id().await?;
            vm_state.preferred = last_accepted_blk_id;
            log::info!("intialized Vm with last accepted block {last_accepted_blk_id}");
        } else {
            let mut genesis_block = Block::new(
                ids::Id::empty(),
                0,
                0,
                vm_state.genesis.data.as_bytes().to_vec(),
                choices::status::Status::default(),
            )?;
            genesis_block.set_state(state.clone());
            genesis_block.accept().await?;

            let genesis_blk_id = genesis_block.id();
            vm_state.preferred = genesis_blk_id;
            log::info!("intialized Vm with genesis block {genesis_blk_id}");
        }

        self.mempool = Arc::new(RwLock::new(VecDeque::with_capacity(100)));
        Ok(())
    }

    /// Called when the node is shutting down.
    async fn shutdown(&self) -> io::Result<()> {
        // grpc servers are shutdown via broadcast channel
        // if additional shutdown is required we can extend.
        Ok(())
    }

    async fn set_state(&self, snow_state: subnet::rpc::snow::State) -> io::Result<()> {
        self.set_state(snow_state).await
    }

    async fn version(&self) -> io::Result<String> {
        Ok(String::from(VERSION))
    }

    async fn create_static_handlers(
        &mut self,
    ) -> io::Result<HashMap<String, subnet::rpc::common::http_handler::HttpHandler>> {
        let svc = api::static_handlers::Service::new(self.clone());
        let mut handler = jsonrpc_core::IoHandler::new();
        handler.extend_with(api::static_handlers::Rpc::to_delegate(svc));

        let http_handler = subnet::rpc::common::http_handler::HttpHandler::new_from_u8(0, handler)
            .map_err(|_| Error::from(ErrorKind::InvalidData))?;

        let mut handlers = HashMap::new();
        handlers.insert("/public".to_string(), http_handler);
        Ok(handlers)
    }

    async fn create_handlers(
        &mut self,
    ) -> io::Result<HashMap<String, subnet::rpc::common::http_handler::HttpHandler>> {
        let svc = api::chain_handlers::Service::new(self.clone());
        let mut handler = jsonrpc_core::IoHandler::new();
        handler.extend_with(api::chain_handlers::Rpc::to_delegate(svc));

        let http_handler = subnet::rpc::common::http_handler::HttpHandler::new_from_u8(0, handler)
            .map_err(|_| Error::from(ErrorKind::InvalidData))?;

        let mut handlers = HashMap::new();
        handlers.insert("/public".to_string(), http_handler);
        Ok(handlers)
    }
}

#[tonic::async_trait]
impl subnet::rpc::snowman::block::ChainVm for Vm {
    async fn build_block(
        &self,
    ) -> io::Result<Box<dyn subnet::rpc::concensus::snowman::Block + Send + Sync>> {
        let mut mempool = self.mempool.write().await;
        if mempool.is_empty() {
            return Err(Error::new(ErrorKind::Other, "no pending block"));
        }

        let vm_state = self.state.read().await;
        if let Some(state) = &vm_state.state {
            let prnt_blk = state.get_block(&vm_state.preferred).await?;
            let unix_now = Utc::now().timestamp() as u64;

            let first = mempool.pop_front().unwrap();
            let mut block = Block::new(
                prnt_blk.id(),
                prnt_blk.height() + 1,
                unix_now,
                first,
                choices::status::Status::Processing,
            )?;
            block.set_state(state.clone());
            block.verify().await?;

            self.notify_block_ready().await;
            return Ok(Box::new(block));
        }

        return Err(Error::new(ErrorKind::NotFound, "state manager not found"));
    }

    async fn set_preference(&self, id: ids::Id) -> io::Result<()> {
        self.set_preference(id).await
    }

    async fn last_accepted(&self) -> io::Result<ids::Id> {
        self.last_accepted().await
    }

    async fn issue_tx(
        &self,
    ) -> io::Result<Box<dyn subnet::rpc::concensus::snowman::Block + Send + Sync>> {
        Err(Error::new(
            ErrorKind::Unsupported,
            "issue_tx not implemented",
        ))
    }
}

#[tonic::async_trait]
impl subnet::rpc::common::apphandler::AppHandler for Vm {
    async fn app_request(
        &self,
        _node_id: &ids::node::Id,
        _request_id: u32,
        _deadline: DateTime<Utc>,
        _request: &[u8],
    ) -> io::Result<()> {
        Err(Error::new(
            ErrorKind::Unsupported,
            "app_request not implemented",
        ))
    }

    async fn app_request_failed(
        &self,
        _node_id: &ids::node::Id,
        _request_id: u32,
    ) -> io::Result<()> {
        Err(Error::new(
            ErrorKind::Unsupported,
            "app_request_failed not implemented",
        ))
    }

    async fn app_response(
        &self,
        _node_id: &ids::node::Id,
        _request_id: u32,
        _response: &[u8],
    ) -> io::Result<()> {
        Err(Error::new(
            ErrorKind::Unsupported,
            "app_response not implemented",
        ))
    }

    async fn app_gossip(&self, _node_id: &ids::node::Id, _msg: &[u8]) -> io::Result<()> {
        Err(Error::new(
            ErrorKind::Unsupported,
            "app_gossip not implemented",
        ))
    }
}

#[tonic::async_trait]
impl subnet::rpc::common::vm::Connector for Vm {
    async fn connected(&self, _id: &ids::node::Id) -> io::Result<()> {
        // no-op
        Ok(())
    }

    async fn disconnected(&self, _id: &ids::node::Id) -> io::Result<()> {
        // no-op
        Ok(())
    }
}

#[tonic::async_trait]
impl subnet::rpc::health::Checkable for Vm {
    async fn health_check(&self) -> io::Result<Vec<u8>> {
        Ok("200".as_bytes().to_vec())
    }
}

#[tonic::async_trait]
impl subnet::rpc::snowman::block::Getter for Vm {
    async fn get_block(
        &self,
        blk_id: ids::Id,
    ) -> io::Result<Box<dyn subnet::rpc::concensus::snowman::Block + Send + Sync>> {
        let vm_state = self.state.read().await;
        if let Some(state) = &vm_state.state {
            let block = state.get_block(&blk_id).await?;
            return Ok(Box::new(block));
        }

        return Err(Error::new(ErrorKind::NotFound, "state manager not found"));
    }
}

#[tonic::async_trait]
impl subnet::rpc::snowman::block::Parser for Vm {
    async fn parse_block(
        &self,
        bytes: &[u8],
    ) -> io::Result<Box<dyn subnet::rpc::concensus::snowman::Block + Send + Sync>> {
        let vm_state = self.state.read().await;
        if let Some(state) = &vm_state.state {
            let mut new_block = Block::from_slice(bytes)?;
            new_block.set_status(choices::status::Status::Processing);
            new_block.set_state(state.clone());
            log::debug!("parsed block {}", new_block.id());

            match state.get_block(&new_block.id()).await {
                Ok(prev) => {
                    log::debug!("returning previously parsed block {}", prev.id());
                    return Ok(Box::new(prev));
                }
                Err(_) => return Ok(Box::new(new_block)),
            };
        }

        return Err(Error::new(ErrorKind::NotFound, "state manager not found"));
    }
}
