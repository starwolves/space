use bevy::prelude::{App, IntoSystemConfigs, Plugin, SystemSet};
use combat::sfx::health_combat_hit_result_sfx;
use entity::entity_types::register_entity_type;
use entity::spawn::build_base_entities;
use physics::spawn::build_rigid_bodies;
use resources::modes::is_server_mode;
use resources::ordering::{ActionsSet, BuildingSet, CombatSet, PreUpdate, Update};

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
    spawn::{build_airlocks, AirlockType},
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum AirLockTimers {
    Timer,
}
pub struct AirLocksPlugin;

impl Plugin for AirLocksPlugin {
    fn build(&self, app: &mut App) {
        if is_server_mode(app) {
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
                    ),
                )
                .add_event::<AirlockLockClosed>()
                .add_event::<AirlockUnlock>();
        }
        app.add_systems(
            PreUpdate,
            (
                build_airlocks::<AirlockType>.in_set(BuildingSet::NormalBuild),
                (build_rigid_bodies::<AirlockType>).in_set(BuildingSet::NormalBuild),
                (build_base_entities::<AirlockType>).in_set(BuildingSet::NormalBuild),
            ),
        );
        register_entity_type::<AirlockType>(app);
    }
}
