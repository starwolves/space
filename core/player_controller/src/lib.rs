//! Logic regarding connected clients.
//! Currently not cleaned up and too large, must be furthered shattered into crates.

/// Manage player boarding.
pub mod boarding;
/// Broadcast unreliable transforms to clients.
mod broadcast_interpolation_transforms;
/// Functions for connection events.
pub mod connection;
/// Manage connection events.
mod connection_events;
/// Manage console commands like rcon authorization.
mod console_commands;
/// Finalize sending entity updates to a player controller.
mod finalize_entity_updates;
/// Manage health UI and its entity updates.
pub mod health_ui;
/// Manage client input.
pub mod input;
/// Manage netcode.
pub mod net;
/// The networking module of this crate.
pub mod networking;
/// Select players with special text character-based queries.
pub mod player_selectors;
/// The Bevy plugin of this crate.
pub mod plugin;
/// Finalize sending netcode messages to a player controller.
mod send_net;
/// Manage the welcome character and role setup UI.
mod setup_ui;
