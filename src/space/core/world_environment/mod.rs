use bevy_app::{App, Plugin};

use self::resources::WorldEnvironment;

pub mod resources;

pub struct WorldEnvironmentPlugin;

impl Plugin for WorldEnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldEnvironment>();
    }
}
