use bevy_app::{App, CoreStage, Plugin};

use self::systems::{
    broadcast_interpolation_transforms::broadcast_interpolation_transforms,
    out_of_bounds_check::out_of_bounds_check, rigidbody_link_transform::rigidbody_link_transform,
};

pub mod components;
pub mod spawn;
pub mod systems;

pub struct RigidBodyPlugin;
impl Plugin for RigidBodyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(out_of_bounds_check)
            .add_system_to_stage(CoreStage::Update, broadcast_interpolation_transforms)
            .add_system(rigidbody_link_transform);
    }
}
