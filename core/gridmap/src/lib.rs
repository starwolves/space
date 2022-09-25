//! The map consisting of cells representing the ship.

/// Setup and build gridmap from data.
pub mod build;
/// Check if an entity can reach another entity.
pub mod can_reach_entity;
/// Manage gridmap events.
pub mod events;
/// Manage gridmap exmination.
mod examine;
/// Manage gridmap FOV.
pub mod fov;
/// Initialize gridmap meta data.
mod init_meta;
/// Manage gridmap netcode.
mod net;
/// The Bevy plugin of this crate.
pub mod plugin;
/// Manage sensing authorization for gridmap examining.
mod sensing_ability;
