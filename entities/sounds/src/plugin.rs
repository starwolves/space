use ambience::ambience_sfx::startup_ambience;
use bevy::prelude::{App, Plugin};
use resources::is_server::is_server;
use crate::ambience;

pub struct SoundsPlugin;

impl Plugin for SoundsPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_startup_system(startup_ambience);
        }
    }
}
