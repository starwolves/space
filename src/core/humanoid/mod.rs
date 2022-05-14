pub mod components;
pub mod entity_update;
pub mod systems;
use bevy_app::{App, Plugin};
use bevy_ecs::schedule::{ParallelSystemDescriptorCoercion, SystemSet};

use bevy_app::CoreStage::PostUpdate;

use self::{entity_update::humanoid_update, systems::humanoid::humanoids};

use super::{PostUpdateLabels, UpdateLabels};

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
