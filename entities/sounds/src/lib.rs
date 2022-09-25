//! Sound effect storage.

use ambience::ambience_sfx::startup_ambience;
use bevy::prelude::{App, Plugin};
use sfx::entity_update::SfxAutoDestroyTimers;

/// Sound library.
pub mod actions;
/// Sound library.
pub mod air_lock;
/// Sound library.
mod ambience;
/// Sound library.
mod combat;
/// Sound library.
pub mod construction;
/// Sound library.
pub mod counter_window;
/// Sound library.
pub mod shared;
/// Sound library.
pub mod ui;

pub struct SoundsPlugin;

impl Plugin for SoundsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(startup_ambience)
            .init_resource::<SfxAutoDestroyTimers>();
    }
}
