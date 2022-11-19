//! Logic regarding connected clients.
//! Currently not cleaned up and too large, must be furthered shattered into crates.

/// Manage client input.
pub mod input;
/// Manage netcode.
pub mod net;
/// The networking module of this crate.
pub mod networking;

/// The configuration send to newly connected clients.
pub mod connections;
/// The pawn controller.
pub mod controller;
/// The Bevy plugin of this crate.
pub mod plugin;
