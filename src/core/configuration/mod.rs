use bevy_app::{App, Plugin};

use self::resources::{ServerId, TickRate, MOTD};

pub mod resources;

pub struct ConfigurationPlugin;

impl Plugin for ConfigurationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TickRate>()
            .init_resource::<ServerId>()
            .init_resource::<MOTD>();
    }
}
