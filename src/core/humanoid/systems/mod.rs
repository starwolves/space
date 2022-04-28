use bevy_app::{App, Plugin};
use bevy_ecs::schedule::{ParallelSystemDescriptorCoercion, SystemSet};
use bevy_rapier3d::physics::PhysicsSystems;

use crate::core::space_plugin::{PostUpdateLabels, UpdateLabels};

use self::humanoid::humanoids;

use super::entity_update::humanoid_update;

pub mod humanoid;
use bevy_app::CoreStage::PostUpdate;

pub struct HumanoidPlugin;

impl Plugin for HumanoidPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            humanoids
                .label(UpdateLabels::StandardCharacters)
                .before(PhysicsSystems::StepWorld)
                .after(UpdateLabels::ProcessMovementInput),
        )
        .add_system_set_to_stage(
            PostUpdate,
            SystemSet::new()
                .label(PostUpdateLabels::EntityUpdate)
                .with_system(humanoid_update),
        );
    }
}
