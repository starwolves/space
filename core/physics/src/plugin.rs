use bevy::prelude::{App, Plugin, PostUpdate, Update};
use bevy_xpbd_3d::prelude::PhysicsPlugins;
use resources::is_server::is_server;

use crate::{
    broadcast_interpolation_transforms::broadcast_interpolation_transforms,
    physics::{disable_rigidbodies, setup_timestep},
    rigidbody_link_transform::rigidbody_link_transform,
};

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_systems(
                Update,
                (rigidbody_link_transform, broadcast_interpolation_transforms),
            );
        }
        app.add_plugins(PhysicsPlugins::default())
            .add_systems(PostUpdate, disable_rigidbodies)
            .add_systems(Update, setup_timestep);
    }
}
