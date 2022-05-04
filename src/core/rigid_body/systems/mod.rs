use bevy_app::{App, CoreStage, Plugin};

pub mod broadcast_interpolation_transforms;
pub mod out_of_bounds_check;
pub mod rigidbody_link_transform;

use broadcast_interpolation_transforms::broadcast_interpolation_transforms;
use out_of_bounds_check::out_of_bounds_check;

pub struct RigidBodyPlugin;

impl Plugin for RigidBodyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(out_of_bounds_check)
            .add_system_to_stage(CoreStage::Update, broadcast_interpolation_transforms);
    }
}
