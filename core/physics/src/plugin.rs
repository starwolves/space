use bevy::prelude::{App, FixedUpdate, IntoSystemConfigs, Plugin};
use bevy_xpbd_3d::prelude::PhysicsPlugins;
use resources::{is_server::is_server, sets::MainSet};

use crate::rigidbody_link_transform::rigidbody_link_transform;

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_systems(
                FixedUpdate,
                rigidbody_link_transform.in_set(MainSet::Update),
            );
        }
        app.add_plugins(PhysicsPlugins::new(FixedUpdate));
    }
}
