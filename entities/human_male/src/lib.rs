//! A humanoid.
//! This entity is always a humanoid and always a pawn.

/// Process player boarding. From setup_ui to spawned into the game.
pub mod boarding;
/// Handler for bare hand combat.
mod hands_attack_handler;
/// The Bevy plugin of this crate.
pub mod plugin;
/// Spawn showcase human male instance for players entering the setup ui.
pub mod setup_ui_showcase;
/// Spawn human male.
pub mod spawn;
