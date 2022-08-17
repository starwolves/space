use api::data::{ActionsLabels, CombatLabels, PostUpdateLabels, UpdateLabels};
use bevy::prelude::{App, ParallelSystemDescriptorCoercion, Plugin, SystemSet};
use combat::{chat::attacked_by_chat, sfx::health_combat_hit_result_sfx};
use networking::messages::net_system;

use crate::{
    examine_events::{examine_entity, ExamineEntityPawn},
    humanoid::{toggle_combat_mode, Humanoid},
};
use bevy::app::CoreStage::PostUpdate;

use super::humanoid::humanoids;
pub struct HumanoidPlugin;

impl Plugin for HumanoidPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            humanoids
                .label(UpdateLabels::StandardCharacters)
                .label(CombatLabels::RegisterAttacks)
                .after(UpdateLabels::ProcessMovementInput),
        )
        .add_system(toggle_combat_mode)
        .add_system(examine_entity.after(ActionsLabels::Action))
        .add_event::<ExamineEntityPawn>()
        .add_system_set_to_stage(
            PostUpdate,
            SystemSet::new()
                .after(PostUpdateLabels::VisibleChecker)
                .label(PostUpdateLabels::Net)
                .with_system(net_system::<ExamineEntityPawn>),
        )
        .add_system(
            health_combat_hit_result_sfx::<Humanoid>.after(CombatLabels::FinalizeApplyDamage),
        )
        .add_system(attacked_by_chat::<Humanoid>.after(CombatLabels::Query));
    }
}
