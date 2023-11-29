//! Sound effects.
//! Works with the entities/sounds crate

/// Build base SFX components.
pub mod builder;
pub mod net;
/// The Bevy plugin of this crate.
pub mod plugin;
/// Resources for proximity messages.
pub mod proximity_message;
/// Resources for radio sounds.
pub mod radio_sound;
/// Manage SFX related timers for auto despawning.
mod timers;
