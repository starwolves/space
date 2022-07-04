use bevy::prelude::{App, Plugin};

use super::environment::WorldEnvironment;

pub struct WorldEnvironmentPlugin;

impl Plugin for WorldEnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldEnvironment>();
    }
}
