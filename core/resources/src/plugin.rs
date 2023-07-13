use bevy::prelude::{App, Plugin};

use crate::{binds::KeyBinds, is_server::is_server};

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        if !is_server() {
            app.init_resource::<KeyBinds>();
        }
    }
}
