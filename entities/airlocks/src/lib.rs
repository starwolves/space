//! Airlocks with authentication systems.
//! Can open, close and be locked.
//! Has a sensor that detect nearby pawn collision and authorization.

/// Air lock actions.
mod actions;
/// On new air lock creation.
mod airlock_added;
/// Manage air lock events.
pub mod airlock_events;
/// Manage air lock timers like auto-close.
mod airlock_tick_timers;
/// Manage air lock entity updates.
mod entity_update;
/// Manage physics events.
mod physics_events;
/// The Bevy plugin of this crate.
pub mod plugin;
/// Air lock resources.
pub mod resources;
/// The air lock spawner.
pub mod spawn;
