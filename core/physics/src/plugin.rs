use bevy::prelude::{App, CoreSet, IntoSystemConfig, Plugin};
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
            app.add_system(rigidbody_link_transform)
                .add_system(broadcast_interpolation_transforms);
        }
        app.add_plugins(PhysicsPlugins)
            .add_system(disable_rigidbodies.in_base_set(CoreSet::PostUpdate))
            .add_startup_system(setup_timestep);
    }
}
