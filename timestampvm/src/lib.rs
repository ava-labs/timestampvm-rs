//! A minimal implementation of custom virtual machine (VM) for Avalanche subnet.
//!
//! This project implements timestampvm that allows anyone to propose and read
//! blocks, each of which is tagged with the proposed timestamp. It implements
//! the snowman block.ChainVM interface in Rust, pluggable to AvalancheGo nodes.
//!
//! See [`ava-labs/timestampvm`](https://github.com/ava-labs/timestampvm) for the original Go implementation.
//!
//! # Layout
//!
//! The project is structured such that it can be used as a template to build
//! more complex VMs (e.g., Ethereum VM, key-value store VM).
//!
//! The major components are:
//!
//! * [`api`](https://docs.rs/timestampvm/latest/timestampvm/api): Implementation of timestampvm APIs.
//! * [`bin/timestampvm`](https://github.com/ava-labs/timestampvm-rs/tree/main/timestampvm/src/bin/timestampvm): Command-line interface, and plugin server.
//! * [`block`](https://docs.rs/timestampvm/latest/timestampvm/block): Implementation of [`snowman.Block`](https://pkg.go.dev/github.com/ava-labs/avalanchego/snow/consensus/snowman#Block) interface for timestampvm.
//! * [`client`](https://docs.rs/timestampvm/latest/timestampvm/client): Implements client for timestampvm APIs.
//! * [`genesis`](https://docs.rs/timestampvm/latest/timestampvm/genesis): Defines timestampvm genesis block.
//! * [`state`](https://docs.rs/timestampvm/latest/timestampvm/state): Manages the virtual machine states.
//! * [`vm`](https://docs.rs/timestampvm/latest/timestampvm/vm): Implementation of [`snowman.block.ChainVM`](https://pkg.go.dev/github.com/ava-labs/avalanchego/snow/engine/snowman/block#ChainVM) interface for timestampvm.
//!
//! ## Example
//!
//! A simple example that prepares an HTTP/1 connection over a Tokio TCP stream.
//!
//! ```no_run
//! use std::io;
//!
//! use timestampvm::vm;
//! use tokio::sync::broadcast::{Receiver, Sender};
//!
//! #[tokio::main]
//! async fn main() -> io::Result<()> {
//!     // ref. https://github.com/env-logger-rs/env_logger/issues/47
//!     env_logger::init_from_env(
//!         env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
//!     );
//!
//!     log::info!("starting timestampvm");
//!
//!     let (stop_ch_tx, stop_ch_rx): (Sender<()>, Receiver<()>) =
//!         tokio::sync::broadcast::channel(1);
//!
//!     let vm_server =
//!         avalanche_types::subnet::rpc::vm::server::Server::new(vm::Vm::new(), stop_ch_tx);
//!
//!     avalanche_types::subnet::rpc::plugin::serve(vm_server, stop_ch_rx).await
//! }
//! ```

pub mod api;
pub mod block;
pub mod client;
pub mod genesis;
pub mod state;
pub mod vm;
