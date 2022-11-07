//! A minimal implementation of custom virtual machine (VM) for Avalanche subnet.
//!
//! This project implements timestampvm that allows anyone to propose and read
//! blocks, each of which is tagged with the proposed timestamp. It implements
//! the snowman block.ChainVM interface in Rust, pluggable to AvalancheGo nodes.
//! See https://github.com/ava-labs/timestampvm for the original Go implementation.
//!
//! # Layout
//!
//! The project is structured such that it can be used as a template to build
//! more complex VMs (e.g., Ethereum VM, key-value store VM).
//!
//! The major components are:
//!
//! * `api`: Implementation of timestampvm APIs.
//!
//! * `bin/timestampvm`: Command-line interface, and plugin server.
//!
//! * `block`: Implementation of snowman.Block interface for timestampvm.
//!
//! * `client`: Implements client for timestampvm API.
//!
//! * `genesis`: Defines timestampvm genesis block.
//!
//! * `state`: Manages the virtual machine states.
//!
//! * `vm`: Implementation of snowman block.ChainVM interface for timestampvm.

pub mod api;
pub mod block;
pub mod client;
pub mod genesis;
pub mod state;
pub mod vm;
