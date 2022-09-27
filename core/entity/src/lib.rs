//! Entity base.
//! Contains logic and resources that interacts with most if not all entities.
//! Includes entity spawning commands through the console.

/// Manage entity related console commands.
pub mod commands;
/// Manage base entity data.
pub mod entity_data;
/// Entity initialization.
mod init;
/// The Bevy plugin of this crate.
pub mod plugin;
/// Base spawner for entities.
pub mod spawn;
