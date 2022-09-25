//! Airlocks with authentication systems.

/// Air lock actions.
mod actions;
/// On new air lock creation.
pub mod air_lock_added;
/// Manage air lock events.
pub mod air_lock_events;
/// Manage air lock timers like auto-close.
mod air_lock_tick_timers;
/// Manage air lock entity updates.
mod entity_update;
/// Manage air lock net code.
mod net;
/// Manage physics events.
mod physics_events;
/// The Bevy plugin of this crate.
pub mod plugin;
/// Air lock resources.
pub mod resources;
/// The air lock spawner.
pub mod spawn;
