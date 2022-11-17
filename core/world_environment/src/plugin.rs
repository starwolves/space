use std::env;

use bevy::prelude::{App, Plugin};

use crate::environment::{startup_environment, WorldEnvironment};

pub struct WorldEnvironmentPlugin;

impl Plugin for WorldEnvironmentPlugin {
    fn build(&self, app: &mut App) {
        if env::var("CARGO_MANIFEST_DIR").unwrap().ends_with("server") {
            app.init_resource::<WorldEnvironment>()
                .add_startup_system(startup_environment);
        }
    }
}
