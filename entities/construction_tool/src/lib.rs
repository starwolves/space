//! Construction tool.
//! Can construct and deconstruct ship cells, interact with the gridmap.

/// Manage construction tool actions.
mod action;
/// Manage consturction tools.
pub mod construction_tool;
pub mod map_construction;
/// The Bevy plugin of this crate.
pub mod plugin;
/// The construction tool spawner.
pub mod spawn;
