//! Manages gravity, temperature, pressure etc.
//! Diffusion takes place at a certain tick rate in which the atmospherics values get updated of each tile since atmospherics is tile bound and calculated per tile.
//! Diffusion is not yet very optimized and even when the atmospheric tiles are in a stabilized state it will continue to recalculate the atmospherics of each active tile.
//! Tiles can have atmospherics effects, for example when the hull at that part is missing or has holes and is constantly draining temperature and pressure.
//! There is integration with the mini-map and the atmospherics data overlays it offers.
//! Tiles with atmospherics can also be examined and have their atmospherics data displayed to the examiner.
//! When players find themselves in unlivable atmospherics conditions they will get to see label warnings.
//! Supports zero gravity for entities.

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
