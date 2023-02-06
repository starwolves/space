use bevy::prelude::{App, Plugin};

use bevy_atmosphere::prelude::AtmospherePlugin;
use resources::is_server::is_server;

use crate::atmosphere::add_atmosphere;
pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        if !is_server() {
            app.add_plugin(AtmospherePlugin)
                .add_startup_system(add_atmosphere);
        }
    }
}
