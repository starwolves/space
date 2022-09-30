//! Interact with inventory entities.
//! For individual entities that can be put inside inventory containers.
//! Includes combat components for inventory items.

/// Manage actions related to inventory items.
mod actions;
/// Combat resources of items.
pub mod combat;
/// Manage entity updates for inventory items.
pub mod entity_update;
/// Components.
pub mod item;
/// The Bevy plugin of this crate.
pub mod plugin;
/// Base spawner of inventory items.
pub mod spawn;
