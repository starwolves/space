use bevy::prelude::{App, FixedUpdate, IntoSystemConfigs, Plugin};
use bevy_xpbd_3d::prelude::PhysicsPlugins;
use resources::{is_server::is_server, sets::MainSet};

use crate::{
    entity::{remove_linked_entities, remove_rigidbodies, RigidBodies},
    mirror_physics_transform::rigidbody_link_transform,
};

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_systems(
                FixedUpdate,
                rigidbody_link_transform.in_set(MainSet::Update),
            );
        }
        app.add_plugins(PhysicsPlugins::new(FixedUpdate))
            .init_resource::<RigidBodies>()
            .add_systems(
                FixedUpdate,
                (remove_linked_entities, remove_rigidbodies).in_set(MainSet::PostUpdate),
            );
    }
}
