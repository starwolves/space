use bevy::prelude::{App, IntoSystemDescriptor, Plugin, SystemSet};
use combat::{chat::attacked_by_chat, sfx::health_combat_hit_result_sfx};
use player::names::UsedNames;
use resources::{
    is_server::is_server,
    labels::{ActionsLabels, CombatLabels, PostUpdateLabels, UpdateLabels},
};

use crate::{
    entity_update::humanoid_core_entity_updates,
    examine_events::examine_entity,
    humanoid::{humanoid_controller_input, mouse_direction_update, toggle_combat_mode, Humanoid},
    thrown_item::thrown_item_adjust_facingdirection,
};
use bevy::app::CoreStage::PostUpdate;

use super::humanoid::humanoid_core;
pub struct HumanoidPlugin;

impl Plugin for HumanoidPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
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
            .init_resource::<UsedNames>()
            .add_system(
                health_combat_hit_result_sfx::<Humanoid>.after(CombatLabels::FinalizeApplyDamage),
            )
            .add_system(attacked_by_chat::<Humanoid>.after(CombatLabels::Query))
            .add_system(mouse_direction_update.before(UpdateLabels::StandardCharacters))
            .add_system(humanoid_controller_input.before(UpdateLabels::StandardCharacters))
            .add_system(thrown_item_adjust_facingdirection);
        }
    }
}
