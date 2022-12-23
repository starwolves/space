//! Entity base.
//! Contains logic and resources that interacts with most if not all entities.
//! Includes the entity spawning base and entity spawn commands through the console.
//! Includes field of view checking for both senser and sensable entities.
//! Includes health systems and resource.
//! Includes the base examine action.

/// Manage base entity data.
pub mod entity_data;
pub mod entity_types;
/// Resources for the ability to examine entities as an action.
pub mod examine;
/// Finalize sending entity updates to a player controller.
mod finalize_entity_updates;
/// Base health resources for entities.
pub mod health;
/// Entity initialization.
pub mod init;
/// Load and unload logic for entities between the server and client.
/// For example based on whether entities are inside the FOV of a player the server may request to (un)load them on the client-side.
/// Entity loading and unloading is just replicated client-side spawning and despawning.
pub mod loading;
/// Meta resources for entities.
pub mod meta;
/// The networking module of this crate.
pub mod net;
/// The Bevy plugin of this crate.
pub mod plugin;
/// Entity that can be sensed, heard or seen by other sensers.
pub mod sensable;
/// Entity that can sense entities that are sensable.
pub mod senser;
/// Showcase resources.
pub mod showcase;
/// Base spawner for entities.
pub mod spawn;
/// Despawns entities.
pub mod spawning_events;
/// Crate that performs FOV logic for sensing and sensable entities.
pub mod visible_checker;
