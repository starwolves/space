//! Crate containing providing widely used global resources.

/// Core server resources.
pub mod core;

/// Systems ordering labels.
pub mod labels;

/// Convert data, mainly used for old  Godot prototype.
pub mod converters;
/// The Bevy ECS plugin of this crate.
pub mod plugin;
// Set window icon of client.
pub mod content;
pub mod grid;
pub mod is_server;
pub mod math;
pub mod set_icon;
