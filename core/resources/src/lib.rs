//! Crate containing providing widely used global resources.

/// Core server resources.
pub mod core;

/// Systems ordering labels.
pub mod labels;

pub mod content;
/// Convert data, mainly used for old  Godot prototype.
pub mod converters;
pub mod grid;
pub mod hud;
pub mod is_server;
pub mod math;
/// The Bevy ECS plugin of this crate.
pub mod plugin;
pub mod set_icon;
pub mod ui;
