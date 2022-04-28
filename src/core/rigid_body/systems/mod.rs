use bevy_app::{App, Plugin};
use bevy_ecs::schedule::ParallelSystemDescriptorCoercion;
use bevy_rapier3d::physics::{PhysicsStages, PhysicsSystems};

pub mod broadcast_interpolation_transforms;
pub mod out_of_bounds_check;
pub mod rigidbody_link_transform;

use broadcast_interpolation_transforms::broadcast_interpolation_transforms;
use out_of_bounds_check::out_of_bounds_check;
use rigidbody_link_transform::rigidbody_link_transform;

use crate::core::space_plugin::UpdateLabels;

pub struct RigidBodyPlugin;

impl Plugin for RigidBodyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(out_of_bounds_check)
            .add_system_to_stage(
                PhysicsStages::SyncTransforms,
                broadcast_interpolation_transforms.after(PhysicsSystems::SyncTransforms),
            )
            .add_system(rigidbody_link_transform.after(UpdateLabels::DropCurrentItem));
    }
}
