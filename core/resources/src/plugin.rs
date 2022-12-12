use std::env;

use bevy::prelude::{App, Plugin};

use crate::{core::ServerId, set_icon::set_window_icon};

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        if env::var("CARGO_MANIFEST_DIR").unwrap().ends_with("server") {
            app.init_resource::<ServerId>();
        } else {
            app.add_startup_system(set_window_icon);
        }
    }
}
