use bevy::prelude::{App, ParallelSystemDescriptorCoercion, Plugin, SystemSet};
use combat::{chat::attacked_by_chat, sfx::health_combat_hit_result_sfx};
use networking::server::net_system;
use pawn::pawn::UsedNames;
use server_instance::labels::{ActionsLabels, CombatLabels, PostUpdateLabels, UpdateLabels};

use crate::{
    entity_update::humanoid_core_entity_updates,
    examine_events::{examine_entity, ExamineEntityPawn},
    humanoid::{toggle_combat_mode, Humanoid},
    user_name::{user_name, NetHumanoid},
};
use bevy::app::CoreStage::PostUpdate;

use super::humanoid::humanoid_core;
pub struct HumanoidPlugin;

impl Plugin for HumanoidPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(feature = "server") {
            app.add_system(
                humanoid_core
                    .label(UpdateLabels::StandardCharacters)
                    .label(CombatLabels::RegisterAttacks)
                    .after(UpdateLabels::ProcessMovementInput),
            )
            .add_system_set_to_stage(
                PostUpdate,
                SystemSet::new()
                    .label(PostUpdateLabels::EntityUpdate)
                    .with_system(humanoid_core_entity_updates),
            )
            .add_system(toggle_combat_mode)
            .add_system(examine_entity.after(ActionsLabels::Action))
            .add_event::<ExamineEntityPawn>()
            .init_resource::<UsedNames>()
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
            .add_system(attacked_by_chat::<Humanoid>.after(CombatLabels::Query))
            .add_system(user_name)
            .add_event::<NetHumanoid>()
            .add_system_set_to_stage(
                PostUpdate,
                SystemSet::new()
                    .after(PostUpdateLabels::VisibleChecker)
                    .label(PostUpdateLabels::Net)
                    .with_system(net_system::<NetHumanoid>),
            );
        }
    }
}
