use bevy_app::{App, Plugin};

use self::systems::{find_path::find_path, steer::steer};

pub mod components;
pub mod functions;
pub mod systems;

pub struct ArtificialUnintelligencePlugin;

impl Plugin for ArtificialUnintelligencePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(find_path).add_system(steer);
    }
}
