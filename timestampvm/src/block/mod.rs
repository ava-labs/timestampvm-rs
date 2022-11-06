use std::{
    fmt,
    io::{self, Error, ErrorKind},
};

use crate::state;
use avalanche_types::{choices, codec::serde::hex_0x_bytes::Hex0xBytes, ids, subnet};
use derivative::{self, Derivative};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Derivative)]
#[derivative(Debug, PartialEq, Eq)]
pub struct Block {
    /// The block Id of the parent block.
    parent_id: ids::Id,
    /// This block's height.
    /// The height of the genesis block is 0.
    height: u64,
    /// Unix second when this block was proposed.
    timestamp: u64,
    /// Arbitrary data.
    #[serde_as(as = "Hex0xBytes")]
    data: Vec<u8>,

    /// Current block status.
    #[serde(skip)]
    status: choices::status::Status,
    /// This block's encoded bytes.
    #[serde(skip)]
    bytes: Vec<u8>,
    /// Generated block Id.
    #[serde(skip)]
    id: ids::Id,

    /// Reference to the Vm state manager for blocks.
    #[derivative(Debug = "ignore", PartialEq = "ignore")]
    #[serde(skip)]
    state: state::State,
}

impl Default for Block {
    fn default() -> Self {
        Self::default()
    }
}

impl Block {
    pub fn default() -> Self {
        Self {
            parent_id: ids::Id::empty(),
            height: 0,
            timestamp: 0,
            data: Vec::new(),

            status: choices::status::Status::default(),
            bytes: Vec::new(),
            id: ids::Id::empty(),

            state: state::State::default(),
        }
    }

    pub fn new(
        parent_id: ids::Id,
        height: u64,
        timestamp: u64,
        data: Vec<u8>,
        status: choices::status::Status,
    ) -> io::Result<Self> {
        let mut b = Self {
            parent_id,
            height,
            timestamp,
            data,
            ..Default::default()
        };

        b.status = status;
        b.bytes = b.to_slice()?;
        b.id = ids::Id::sha256(&b.bytes);

        Ok(b)
    }

    pub fn to_json_string(&self) -> io::Result<String> {
        serde_json::to_string(&self).map_err(|e| {
            Error::new(
                ErrorKind::Other,
                format!("failed to serialize Block to JSON string {}", e),
            )
        })
    }

    pub fn to_slice(&self) -> io::Result<Vec<u8>> {
        serde_json::to_vec(&self).map_err(|e| {
            Error::new(
                ErrorKind::Other,
                format!("failed to serialize Block to JSON bytes {}", e),
            )
        })
    }

    pub fn from_slice(d: impl AsRef<[u8]>) -> io::Result<Self> {
        let dd = d.as_ref();
        let mut b: Self = serde_json::from_slice(dd).map_err(|e| {
            Error::new(
                ErrorKind::Other,
                format!("failed to deserialize Block from JSON {}", e),
            )
        })?;

        b.bytes = dd.to_vec();
        b.id = ids::Id::sha256(&b.bytes);

        Ok(b)
    }

    pub fn parent_id(&self) -> ids::Id {
        self.parent_id
    }

    pub fn height(&self) -> u64 {
        self.height
    }

    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn status(&self) -> choices::status::Status {
        self.status.clone()
    }

    pub fn set_status(&mut self, status: choices::status::Status) {
        self.status = status;
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn id(&self) -> ids::Id {
        self.id
    }

    pub fn set_state(&mut self, state: state::State) {
        self.state = state;
    }

    pub async fn verify(&mut self) -> io::Result<()> {
        if self.height == 0 && self.parent_id == ids::Id::empty() {
            log::debug!(
                "block {} has an empty parent Id since it's a genesis block -- skipping verify",
                self.id
            );
            return Ok(());
        }

        // if already exists in database, it means it's already accepted
        // thus no need to verify once more
        // TODO: cache verified blocks in memory
        if self.state.get_block(&self.id).await.is_ok() {
            log::debug!("block {} already verified", self.id);
            return Ok(());
        }

        let prnt_blk = self.state.get_block(&self.parent_id).await?;

        if prnt_blk.height != self.height - 1 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!(
                    "parent block height {} != current block height {} - 1",
                    prnt_blk.height, self.height
                ),
            ));
        }

        if prnt_blk.timestamp > self.timestamp {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!(
                    "parent block timestamp {} > current block timestamp {}",
                    prnt_blk.timestamp, self.timestamp
                ),
            ));
        }

        return Ok(());
    }

    pub async fn accept(&mut self) -> io::Result<()> {
        self.set_status(choices::status::Status::Accepted);

        // only decided blocks are persistent -- no reorg
        self.state.put_block(&self.clone()).await?;
        self.state.set_last_accepted_block(&self.id()).await
    }

    pub async fn reject(&mut self) -> io::Result<()> {
        self.set_status(choices::status::Status::Rejected);

        // only decided blocks are persistent -- no reorg
        self.state.put_block(&self.clone()).await
    }
}

/// ref. https://doc.rust-lang.org/std/string/trait.ToString.html
/// ref. https://doc.rust-lang.org/std/fmt/trait.Display.html
/// Use "Self.to_string()" to directly invoke this
impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let serialized = self.to_json_string().unwrap();
        write!(f, "{serialized}")
    }
}

/// RUST_LOG=debug cargo test --package timestampvm --lib -- block::test_block --exact --show-output
#[tokio::test]
async fn test_block() {
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .is_test(true)
        .try_init();

    let mut genesis_blk = Block::new(
        ids::Id::empty(),
        0,
        random_manager::u64(),
        random_manager::bytes(10).unwrap(),
        choices::status::Status::default(),
    )
    .unwrap();
    log::info!("deserialized: {genesis_blk} (block Id: {})", genesis_blk.id);

    let serialized = genesis_blk.to_slice().unwrap();
    let deserialized = Block::from_slice(&serialized).unwrap();
    log::info!("deserialized: {deserialized}");

    assert_eq!(genesis_blk, deserialized);

    let state = state::State::default();
    assert!(!state.has_last_accepted_block().await.unwrap());

    // inner db instance is protected with arc and mutex
    // so cloning outer struct "State" should implicitly
    // share the db instances
    genesis_blk.set_state(state.clone());

    genesis_blk.verify().await.unwrap();
    genesis_blk.accept().await.unwrap();
    assert_eq!(genesis_blk.status, choices::status::Status::Accepted);
    assert!(state.has_last_accepted_block().await.unwrap());

    let last_accepted_blk_id = state.get_last_accepted_block_id().await.unwrap();
    assert_eq!(last_accepted_blk_id, genesis_blk.id());

    let read_blk = state.get_block(&genesis_blk.id()).await.unwrap();
    assert_eq!(genesis_blk, read_blk);

    let mut blk1 = Block::new(
        genesis_blk.id,
        genesis_blk.height + 1,
        genesis_blk.timestamp + 1,
        random_manager::bytes(10).unwrap(),
        choices::status::Status::default(),
    )
    .unwrap();
    log::info!("blk1: {blk1}");
    blk1.set_state(state.clone());

    blk1.verify().await.unwrap();
    blk1.accept().await.unwrap();
    assert_eq!(blk1.status, choices::status::Status::Accepted);

    let last_accepted_blk_id = state.get_last_accepted_block_id().await.unwrap();
    assert_eq!(last_accepted_blk_id, blk1.id());

    let read_blk = state.get_block(&blk1.id()).await.unwrap();
    assert_eq!(blk1, read_blk);

    let mut blk2 = Block::new(
        blk1.id,
        blk1.height + 1,
        blk1.timestamp + 1,
        random_manager::bytes(10).unwrap(),
        choices::status::Status::default(),
    )
    .unwrap();
    log::info!("blk2: {blk2}");
    blk2.set_state(state.clone());

    blk2.verify().await.unwrap();
    blk2.reject().await.unwrap();
    assert_eq!(blk2.status, choices::status::Status::Rejected);

    // "blk2" is rejected, so last accepted block must be "blk1"
    let last_accepted_blk_id = state.get_last_accepted_block_id().await.unwrap();
    assert_eq!(last_accepted_blk_id, blk1.id());

    let read_blk = state.get_block(&blk2.id()).await.unwrap();
    assert_eq!(blk2, read_blk);

    let mut blk3 = Block::new(
        blk2.id,
        blk2.height - 1,
        blk2.timestamp + 1,
        random_manager::bytes(10).unwrap(),
        choices::status::Status::default(),
    )
    .unwrap();
    log::info!("blk3: {blk3}");
    blk3.set_state(state.clone());

    assert!(blk3.verify().await.is_err());

    assert!(state.has_last_accepted_block().await.unwrap());
}

#[tonic::async_trait]
impl subnet::rpc::concensus::snowman::Block for Block {
    async fn bytes(&self) -> &[u8] {
        return self.bytes.as_ref();
    }

    async fn to_bytes(&self) -> io::Result<Vec<u8>> {
        self.to_slice()
    }

    async fn height(&self) -> u64 {
        self.height
    }

    async fn timestamp(&self) -> u64 {
        self.timestamp
    }

    async fn parent(&self) -> ids::Id {
        self.parent_id.clone()
    }

    async fn verify(&mut self) -> io::Result<()> {
        self.verify().await
    }
}

#[tonic::async_trait]
impl subnet::rpc::concensus::snowman::Decidable for Block {
    /// Implements "snowman.Block.choices.Decidable"
    async fn status(&self) -> choices::status::Status {
        self.status.clone()
    }

    async fn id(&self) -> ids::Id {
        self.id
    }

    async fn accept(&mut self) -> io::Result<()> {
        self.accept().await
    }

    async fn reject(&mut self) -> io::Result<()> {
        self.reject().await
    }
}

#[tonic::async_trait]
impl subnet::rpc::concensus::snowman::Initializer for Block {
    async fn init(&mut self, bytes: &[u8], status: choices::status::Status) -> io::Result<()> {
        *self = Block::from_slice(bytes)?;
        self.status = status;

        Ok(())
    }
}

#[tonic::async_trait]
impl subnet::rpc::concensus::snowman::StatusWriter for Block {
    async fn set_status(&mut self, status: choices::status::Status) {
        self.set_status(status)
    }
}
