//! Atmospherics manages gravity, temperature, pressure etc.

/// Calculate and simulate atmospherics for tiles.
pub mod diffusion;
/// Step atmospherics effects of tiles.
mod effects;
/// For atmospherics related data in the examine action.
mod examine_events;
/// Startup atmospherics.
mod init;
/// Manage UI input related to the mini-map with atmospherics overlays.
mod map_events;
/// Manage netcode.
mod net;
/// Manage visual warning indicators for clients with relevant atmospherics notices.
mod notices;
/// The Bevy plugin of the crate.
pub mod plugin;
/// Update atmospherics at cell removal event.
mod remove_cell_atmos_event;
/// Apply the accumulated forces calculated by this crate to the rigid bodies.
mod rigidbody_forces;
/// Authorization check if a pawn can examine and see atmospherics data.
mod sensing_ability;
/// Manage zero gravity.
mod zero_gravity;
