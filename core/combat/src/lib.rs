//! For combat and damage applications.

/// Cache active attacks.
pub mod active_attacks;
/// Manage damage applications for combat and health.
mod apply_damage;
/// The attack event.
pub mod attack;
/// Hooks text in chat.
pub mod chat;
/// Manage visual laser projectiles for clients.
pub mod laser_visuals;
/// Physics queries for melee combat.
pub mod melee_queries;
/// The Bevy plugin of this crate.
pub mod plugin;
/// Physics queries for projectile combat.
pub mod projectile_queries;
/// Hooks for sound effects.
pub mod sfx;
