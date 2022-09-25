//! Humanoid entities share animations and API.

/// Hooks for examining humanoid entities.
mod examine_events;
/// Perform core humanoid logic, including animation handling, state management and more.
pub mod humanoid;
/// The Bevy plugin of this crate.
pub mod plugin;
/// Manage humanoid user names.
pub mod user_name;
