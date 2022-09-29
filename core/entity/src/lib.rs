//! Entity base.
//! Contains logic and resources that interacts with most if not all entities.
//! Includes the entity spawning base and entity spawn commands through the console.

/// Manage base entity data.
pub mod entity_data;
/// Entity initialization.
mod init;
/// Meta resources for entities.
pub mod meta;
/// The Bevy plugin of this crate.
pub mod plugin;
/// Base spawner for entities.
pub mod spawn;
