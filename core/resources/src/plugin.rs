use bevy::prelude::{App, FixedUpdate, IntoSystemSetConfigs, Plugin};
use bevy_xpbd_3d::PhysicsSet;

use crate::{binds::KeyBinds, is_server::is_server, sets::MainSet};

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        if !is_server() {
            app.init_resource::<KeyBinds>();
        }

        app.configure_sets(
            FixedUpdate,
            (MainSet::PreUpdate, MainSet::Update, MainSet::PostUpdate)
                .chain()
                .before(PhysicsSet::Prepare),
        );
    }
}
