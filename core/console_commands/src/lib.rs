//! Console commands are a powerful way to remotely interact with the server as an admin or regular player.
//! Custom commands can be added and configured server-side.

/// Resources and initialization.
pub mod commands;
/// Send configuration to newly connected clients.
pub mod connections;
/// Initialize console commands.
pub mod init;
/// The networking module of this crate.
pub mod networking;
/// The Bevy plugin of this crate.
pub mod plugins;
