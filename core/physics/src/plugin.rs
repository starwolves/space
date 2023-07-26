use bevy::prelude::{App, FixedUpdate, IntoSystemConfigs, Plugin};
use bevy_xpbd_3d::prelude::PhysicsPlugins;
use resources::{is_server::is_server, sets::MainSet};

use crate::{
    broadcast_interpolation_transforms::broadcast_interpolation_transforms,
    physics::disable_rigidbodies, rigidbody_link_transform::rigidbody_link_transform,
};

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_systems(
                FixedUpdate,
                (rigidbody_link_transform, broadcast_interpolation_transforms)
                    .in_set(MainSet::Update),
            );
        }
        app.add_plugins(PhysicsPlugins::new(FixedUpdate))
            .add_systems(FixedUpdate, disable_rigidbodies.in_set(MainSet::PostUpdate));
    }
}
