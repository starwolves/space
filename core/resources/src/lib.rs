//! Crate containing providing widely used global resources.

/// Core server resources.
pub mod core;

/// Systems ordering labels.
pub mod sets;

pub mod content;
/// Convert data, mainly used for old  Godot prototype.
pub mod converters;
pub mod correction;
pub mod grid;
pub mod hud;
pub mod input;
pub mod math;
pub mod modes;
pub mod physics;
pub mod player;
/// The Bevy ECS plugin of this crate.
pub mod plugin;
pub mod set_icon;
pub mod ui;
