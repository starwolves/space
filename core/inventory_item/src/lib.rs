//! Interact with inventory entities.
//! For individual entities that can be put inside inventory containers with the standard actions applied to them.

/// Manage actions related to inventory items.
mod actions;
/// Manage entity updates for inventory items.
pub mod entity_update;
/// Components.
pub mod item;
/// The Bevy plugin of this crate.
pub mod plugin;
/// Base spawner of inventory items.
pub mod spawn;
