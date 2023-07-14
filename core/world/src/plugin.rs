use bevy::prelude::{App, Plugin};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, _app: &mut App) {
        /*if !is_server() {
            app.add_plugins(AtmospherePlugin)
                .add_systems(Startup, add_atmosphere);
        }*/
    }
}
