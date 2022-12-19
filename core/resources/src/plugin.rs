use bevy::prelude::{App, Plugin};

use crate::{core::ServerId, is_server::is_server, set_icon::set_window_icon};

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.init_resource::<ServerId>();
        } else {
            app.add_startup_system(set_window_icon);
        }
    }
}
