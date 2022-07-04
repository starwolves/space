use bevy::prelude::{App, ParallelSystemDescriptorCoercion, Plugin, SystemSet};

use crate::core::space_plugin::plugin::{PostUpdateLabels, UpdateLabels};

use super::{entity_update::humanoid_update, humanoid::humanoids};
use bevy::app::CoreStage::PostUpdate;

pub struct HumanoidPlugin;

impl Plugin for HumanoidPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            humanoids
                .label(UpdateLabels::StandardCharacters)
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
