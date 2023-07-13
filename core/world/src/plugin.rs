use bevy::prelude::{App, Plugin, Startup};

use bevy_atmosphere::prelude::AtmospherePlugin;
use resources::is_server::is_server;

use crate::atmosphere::add_atmosphere;
pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        if !is_server() {
            app.add_plugins(AtmospherePlugin)
                .add_systems(Startup, add_atmosphere);
        }
    }
}
