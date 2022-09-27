//! Health for entities and gridmap cells.
//! Gridmap cells have their health stored in gridmap layer resources whereas entities have their health data stored as components.
//! Integrates with the combat crate.

/// Hooks for examining entities with health component.
mod examine_events;
/// The Bevy plugin of this crate.
pub mod plugin;
