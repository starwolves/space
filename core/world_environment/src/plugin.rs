use bevy::prelude::{App, Plugin};
use api::world_environment::WorldEnvironment;

use crate::environment::startup_environment;

pub struct WorldEnvironmentPlugin;

impl Plugin for WorldEnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldEnvironment>()
            .add_startup_system(startup_environment);
    }
}
