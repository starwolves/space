//! Netcode.
//! Contains most client-side Input events.
//! Where the server starts and the listener gets configured.

/// General client-side server input manager.
pub mod client;
/// The Bevy plugin of this crate.
pub mod plugin;
/// General server-side client input manager.
pub mod server;
/// Produces identifiers for netcode message types so netcode messages can be created from crates and get uniquely serialized.
pub mod typenames;
