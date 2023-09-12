use bevy::prelude::{App, FixedUpdate, IntoSystemConfigs, Plugin};
use combat::{chat::attacked_by_chat, sfx::health_combat_hit_result_sfx};
use controller::input::Controller;
use player::names::UsedNames;
use resources::{
    is_server::is_server,
    sets::{ActionsSet, CombatSet, MainSet, UpdateSet},
};

use crate::{
    examine_events::examine_entity,
    humanoid::{humanoid_movement, Humanoid},
};

pub struct HumanoidPlugin;

impl Plugin for HumanoidPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app /*.add_system(
                    humanoid_core
                        .label(UpdateLabels::StandardCharacters)
                        .label(CombatLabels::RegisterAttacks)
                        .after(UpdateLabels::ProcessMovementInput),
                )*/
                .add_systems(
                    FixedUpdate,
                    (
                        examine_entity.after(ActionsSet::Action),
                        health_combat_hit_result_sfx::<Humanoid>
                            .after(CombatSet::FinalizeApplyDamage),
                        attacked_by_chat::<Humanoid>.after(CombatSet::Query),
                    )
                        .in_set(MainSet::Update),
                )
                .init_resource::<UsedNames>();
        }
        app.add_systems(
            FixedUpdate,
            humanoid_movement
                .in_set(UpdateSet::StandardCharacters)
                .in_set(MainSet::Update)
                .after(Controller::Input),
        );
    }
}
