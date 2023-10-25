use crate::ambience;
use ambience::ambience_sfx::startup_ambience;
use bevy::prelude::{App, Plugin, Startup};
use resources::modes::is_server;

pub struct SoundsPlugin;

impl Plugin for SoundsPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_systems(Startup, startup_ambience);
        }
    }
}
