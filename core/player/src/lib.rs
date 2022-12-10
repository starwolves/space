//! Crate manages several player specific logic, such as loading in and processing the character setup UI.

/// Manage supplied account name of connections.
pub mod account;
/// Manage player boarding.
pub mod boarding;
/// Configure the client on server connection.
pub mod configuration;
/// The configuration send from the server to newly connected clients.
pub mod connections;
/// Generate human names.
pub mod name_generator;
/// Account and player names.
pub mod names;
/// The Bevy plugin of this crate.
pub mod plugin;
/// Map spawn points.
pub mod spawn_points;
