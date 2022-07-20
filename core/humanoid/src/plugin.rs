use bevy::prelude::{App, ParallelSystemDescriptorCoercion, Plugin, SystemSet};
use networking::messages::net_system;
use shared::{
    data::{PostUpdateLabels, UpdateLabels},
    examinable::ExamineLabels,
};

use crate::{
    examine_events::{examine_entity, ExamineEntityPawn},
    humanoid::toggle_combat_mode,
};
use bevy::app::CoreStage::PostUpdate;

use super::humanoid::humanoids;
pub struct HumanoidPlugin;

impl Plugin for HumanoidPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            humanoids
                .label(UpdateLabels::StandardCharacters)
                .after(UpdateLabels::ProcessMovementInput),
        )
        .add_system(toggle_combat_mode)
        .add_system(examine_entity.after(ExamineLabels::Default))
        .add_event::<ExamineEntityPawn>()
        .add_system_set_to_stage(
            PostUpdate,
            SystemSet::new()
                .after(PostUpdateLabels::VisibleChecker)
                .label(PostUpdateLabels::Net)
                .with_system(net_system::<ExamineEntityPawn>),
        );
    }
}
