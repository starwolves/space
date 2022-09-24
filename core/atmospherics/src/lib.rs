//! Atmospherics manages gravity, temperature, pressure etc.

pub mod diffusion;
mod effects;
mod examine_events;
mod init;
mod map_events;
mod net;
mod notices;
pub mod plugin;
mod remove_cell_atmos_event;
mod rigidbody_forces;
mod sensing_ability;
mod zero_gravity;
