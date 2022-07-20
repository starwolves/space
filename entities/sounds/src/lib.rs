use ambience::ambience_sfx::startup_ambience;
use bevy::prelude::{App, Plugin};
use sfx::entity_update::SfxAutoDestroyTimers;

pub mod actions;
pub mod air_lock;
pub mod ambience;
pub mod combat;
pub mod construction;
pub mod counter_window;
pub mod ui;

pub struct SoundsPlugin;

impl Plugin for SoundsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(startup_ambience)
            .init_resource::<SfxAutoDestroyTimers>();
    }
}

pub mod shared;
