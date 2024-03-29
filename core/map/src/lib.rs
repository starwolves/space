//! The 2D mini-map that can display various data with overlays.

/// Manage overlay changes.
mod change_overlay;
/// Manage connection events.
pub mod connections;
/// Resources.
pub mod map;
/// Manage player input.
pub mod map_input;
/// The networking module of this crate.
pub mod net;
/// The Bevy plugin of this crate.
pub mod plugin;
