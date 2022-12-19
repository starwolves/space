use bevy::prelude::{App, Plugin};
use resources::is_server::is_server;

use crate::environment::{startup_environment, WorldEnvironment};

pub struct WorldEnvironmentPlugin;

impl Plugin for WorldEnvironmentPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.init_resource::<WorldEnvironment>()
                .add_startup_system(startup_environment);
        }
    }
}
