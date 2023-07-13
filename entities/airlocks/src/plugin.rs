use bevy::prelude::{App, IntoSystemConfigs, Plugin, PostUpdate, SystemSet, Update};
use combat::sfx::health_combat_hit_result_sfx;
use entity::entity_types::register_entity_type;
use entity::spawn::build_base_entities;
use physics::spawn::build_rigid_bodies;
use resources::is_server::is_server;
use resources::labels::{ActionsLabels, BuildingLabels, CombatLabels, PostUpdateLabels};

use crate::{
    actions::{
        airlock_actions, build_actions, lock_action_prequisite_check,
        toggle_open_action_prequisite_check,
    },
    airlock_events::{
        AirLockLockOpen, AirlockCollision, AirlockLockClosed, AirlockUnlock, InputAirlockToggleOpen,
    },
    resources::Airlock,
};

use super::{
    airlock_added::{airlock_added, airlock_default_map_added},
    airlock_events::airlock_events,
    airlock_tick_timers::airlock_tick_timers,
    entity_update::airlock_update,
    spawn::{build_airlocks, AirlockType},
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum AirLockTimers {
    Timer,
}
pub struct AirLocksPlugin;

impl Plugin for AirLocksPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_event::<AirlockCollision>()
                .add_event::<InputAirlockToggleOpen>()
                .add_event::<AirLockLockOpen>()
                .add_systems(
                    Update,
                    (
                        airlock_added,
                        airlock_tick_timers,
                        airlock_default_map_added,
                        airlock_events,
                        health_combat_hit_result_sfx::<Airlock>
                            .after(CombatLabels::FinalizeApplyDamage),
                        toggle_open_action_prequisite_check
                            .in_set(ActionsLabels::Approve)
                            .after(ActionsLabels::Build),
                        lock_action_prequisite_check
                            .in_set(ActionsLabels::Approve)
                            .after(ActionsLabels::Build),
                        airlock_actions
                            .in_set(ActionsLabels::Action)
                            .after(ActionsLabels::Approve),
                        build_actions
                            .in_set(ActionsLabels::Build)
                            .after(ActionsLabels::Init),
                    ),
                )
                .add_event::<AirlockLockClosed>()
                .add_event::<AirlockUnlock>()
                .add_systems(
                    PostUpdate,
                    airlock_update.in_set(PostUpdateLabels::EntityUpdate),
                );
        }
        app.add_systems(
            Update,
            (
                build_airlocks::<AirlockType>.after(BuildingLabels::TriggerBuild),
                (build_rigid_bodies::<AirlockType>).after(BuildingLabels::TriggerBuild),
                (build_base_entities::<AirlockType>).after(BuildingLabels::TriggerBuild),
            ),
        );
        register_entity_type::<AirlockType>(app);
    }
}
