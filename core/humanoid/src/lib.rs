//! For entities that share humanoid behaviour, including humanoid animations which are the Mixamo animations.
//! This is not restricted to humans as it could include various other species species with humanoid characters, animations and gameplay.
//! All humanoids are pawns. The current player-controllable implementation of this is the human_male.
//! All humanoids are inventory holders and have their own systems wrapping those events to keep 3d slot attachment and textures up to date with which items are equipped and where.

/// Hooks for examining humanoid entities.
mod examine_events;
/// Perform core humanoid logic, including animation handling, state management and more.
pub mod humanoid;
/// The Bevy plugin of this crate.
pub mod plugin;
/// Manage humanoid user names.
pub mod user_name;
