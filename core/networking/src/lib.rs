//! Netcode.
//! Contains most client-side Input events.
//! Where the server starts and the listener gets configured.

/// General client-side server input manager.
pub mod client;
/// Create reliable and consistent identifiers linked with 16-bit identifiers for netcode messages. Required for modular netcode practises.
pub mod messaging;
/// The Bevy plugin of this crate.
pub mod plugin;
/// General server-side client input manager.
pub mod server;
/// Tickrate synchronization.
pub mod stamp;
