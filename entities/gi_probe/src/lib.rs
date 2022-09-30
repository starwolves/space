//! Manages client-side ambient occlusion.

/// Core resources.
pub mod core;
/// Manage GI probe entity updates.
mod entity_update;
/// The Bevy plugin of this crate.
pub mod plugin;
/// Load from json.
mod process_content;
/// Spawner.
pub mod spawn;
