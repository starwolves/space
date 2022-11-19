//! Crate manages several player specific logic, such as loading in and processing the character setup UI.

/// Manage player boarding.
pub mod boarding;
/// Functions for connection events.
pub mod connection;
/// The configuration send from the server to newly connected clients.
pub mod connections;
/// Account and player names.
pub mod names;
/// The networking module of this crate.
pub mod networking;
/// The Bevy plugin of this crate.
pub mod plugin;
/// Map spawn points.
pub mod spawn_points;

/// Manage the welcome character and role setup UI.
pub mod setup_ui;

/// Generate names.
pub mod name_generator;
