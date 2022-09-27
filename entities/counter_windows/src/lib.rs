//! Interactable windows with authorization systems.
//! Can open, close and be locked.
//! Has a sensor that detect nearby pawn collision and authorization.

/// Manage counter window actions like opening and closing.
mod actions;
/// On new counter window creation.
pub mod counter_window_added;
/// Manage counter window events.
pub mod counter_window_events;
/// Tick counter window timers.
mod counter_window_tick_timers;
/// Manage entity updates.
mod entity_update;
/// The netcode.
mod net;
/// Manage physics events.
pub mod physics_events;
/// The Bevy plugin of this crate.
pub mod plugin;
/// Spawner.
pub mod spawn;
