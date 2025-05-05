
//! Interface with Wi-Fi networks.
//! Currently, only Linux is supported (via `nmcli`).

pub(crate) mod log;

/// WaiFai errors.
pub mod error;
/// Discovered networks.
pub mod network;
pub use network::*;
/// Scan/connect/disconnect operations.
pub mod client;
pub use client::*;
/// Hotspot related actions.
pub mod hotspot;
pub use hotspot::*;
/// Base struct of operations.
pub mod wifi;
pub use wifi::*;
