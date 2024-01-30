use bevy::prelude::{App, IntoSystemConfigs, Plugin};
use combat::{chat::attacked_by_chat, sfx::health_combat_hit_result_sfx};
use controller::input::ControllerSet;
use pawn::camera::LookTransformSet;
use resources::{
    input::InputSet,
    modes::{is_correction_mode, is_server_mode},
    ordering::{ActionsSet, CombatSet, Update, UpdateSet},
};

use crate::{
    examine_events::examine_entity,
    humanoid::{humanoid_movement, Humanoid},
};

pub struct HumanoidPlugin;

impl Plugin for HumanoidPlugin {
    fn build(&self, app: &mut App) {
        if is_server_mode(app) && !is_correction_mode(app) {
            app.add_systems(
                Update,
                (
                    examine_entity.after(ActionsSet::Action),
                    health_combat_hit_result_sfx::<Humanoid>.after(CombatSet::FinalizeApplyDamage),
                    attacked_by_chat::<Humanoid>.after(CombatSet::Query),
                ),
            );
        }
        app.add_systems(
            Update,
            humanoid_movement
                .in_set(UpdateSet::StandardCharacters)
                .after(ControllerSet::Input)
                .after(InputSet::ApplyLiveCache)
                .after(LookTransformSet::Sync),
        );
    }
}
