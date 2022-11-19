use std::env;

use bevy::prelude::{App, Plugin};

use crate::core::ServerId;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        if env::var("CARGO_MANIFEST_DIR").unwrap().ends_with("server") {
            app.init_resource::<ServerId>();
        }
        if cfg!(feature = "client") {}
    }
}
