use std::env;

use bevy::prelude::{App, CoreStage, Plugin};

use crate::{
    broadcast_interpolation_transforms::broadcast_interpolation_transforms,
    out_of_bounds_teleportation::out_of_bounds_tp,
    rigidbody_link_transform::rigidbody_link_transform,
};

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        if env::var("CARGO_MANIFEST_DIR").unwrap().ends_with("server") {
            app.add_system(out_of_bounds_tp)
                .add_system(rigidbody_link_transform)
                .add_system_to_stage(CoreStage::Update, broadcast_interpolation_transforms);
        }
    }
}
