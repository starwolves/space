//! Console commands are a powerful way to remotely interact with the server as an admin or regular player.
//! Custom commands can be added and configured server-side.

/// Resources and initialization.
pub mod commands;
/// Select players with special text character-based queries.
pub mod player_selectors;
/// The Bevy plugin of this crate.
pub mod plugins;
