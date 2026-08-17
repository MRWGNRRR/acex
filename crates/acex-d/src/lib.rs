//! Batteries-included entry point for the ACED diagnostics stack.
//!
//! Alloc-backed by default. For no_std/embedded targets, depend on the
//! individual `acex-*` crates directly instead of this facade.

pub use acex_can as can;
pub use acex_client as client;
pub use acex_core as core;
pub use acex_doip as doip;
pub use acex_gateway as gateway;
pub use acex_macros as macros;
pub use acex_proto as proto;
pub use acex_server as server;
pub use acex_sim as sim;
pub use acex_uds as uds;
