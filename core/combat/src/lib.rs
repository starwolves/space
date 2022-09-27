//! For combat and damage applications.
//! Modular systems to process attacks, including physics attacks, chat hooks and sound effect hooks.
//! Happens in a modular way so correct interaction with the combat systems relies on systems ordering and labels.
//! Damage applications can be modified by systems and inventory item handler systems allow you to configure how certain weapons interact with this crate.
//! Combat events also allow for modular damage flags, health flags, damage multipliers and more.

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
