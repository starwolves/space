use std::env;

use bevy::prelude::{App, Plugin};

use crate::core::{HandleToEntity, ServerId};

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        if env::var("CARGO_MANIFEST_DIR").unwrap().ends_with("server") {
            app.init_resource::<HandleToEntity>()
                .init_resource::<ServerId>();
        }
        if cfg!(feature = "client") {}
    }
}
