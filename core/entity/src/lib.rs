//! Entity base.
//! Contains logic and resources that interacts with most if not all entities.
//! Includes the entity spawning base and entity spawn commands through the console.
//! Includes field of view checking for both senser and sensable entities.
//! Includes health systems and resource.
//! Includes the base examine action.

/// Perform base entity actions such as examining.
pub mod actions;
/// Broadcast unreliable transforms to clients.
mod broadcast_interpolation_transforms;
/// Despawns entities.
pub mod despawn;
/// Manage base entity data.
pub mod entity_data;
/// Resources for the ability to examine entities as an action.
pub mod examine;
/// Finalize sending entity updates to a player controller.
mod finalize_entity_updates;
/// Base health resources for entities.
pub mod health;
/// Entity initialization.
pub mod init;
/// Meta resources for entities.
pub mod meta;
/// The networking module of this crate.
pub mod networking;
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
/// Crate that performs FOV logic for sensing and sensable entities.
pub mod visible_checker;
