use bevy::prelude::{App, FixedUpdate, IntoSystemConfigs, Plugin, SystemSet};
use combat::sfx::health_combat_hit_result_sfx;
use entity::entity_types::register_entity_type;
use entity::spawn::{build_base_entities, SpawnItemSet};
use physics::spawn::build_rigid_bodies;
use resources::is_server::is_server;
use resources::sets::{ActionsSet, CombatSet, MainSet, PostUpdateSet};

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
                    FixedUpdate,
                    (
                        airlock_added,
                        airlock_tick_timers,
                        airlock_default_map_added,
                        airlock_events,
                        health_combat_hit_result_sfx::<Airlock>
                            .after(CombatSet::FinalizeApplyDamage),
                        toggle_open_action_prequisite_check
                            .in_set(ActionsSet::Approve)
                            .after(ActionsSet::Build),
                        lock_action_prequisite_check
                            .in_set(ActionsSet::Approve)
                            .after(ActionsSet::Build),
                        airlock_actions
                            .in_set(ActionsSet::Action)
                            .after(ActionsSet::Approve),
                        build_actions
                            .in_set(ActionsSet::Build)
                            .after(ActionsSet::Init),
                    )
                        .in_set(MainSet::Update),
                )
                .add_event::<AirlockLockClosed>()
                .add_event::<AirlockUnlock>()
                .add_systems(
                    FixedUpdate,
                    airlock_update
                        .in_set(PostUpdateSet::EntityUpdate)
                        .in_set(MainSet::PostUpdate),
                );
        }
        app.add_systems(
            FixedUpdate,
            (
                build_airlocks::<AirlockType>.after(SpawnItemSet::SpawnHeldItem),
                (build_rigid_bodies::<AirlockType>).after(SpawnItemSet::SpawnHeldItem),
                (build_base_entities::<AirlockType>).after(SpawnItemSet::SpawnHeldItem),
            )
                .in_set(MainSet::Update),
        );
        register_entity_type::<AirlockType>(app);
    }
}
