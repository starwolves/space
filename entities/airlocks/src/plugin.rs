use bevy::prelude::{App, IntoSystemDescriptor, Plugin, SystemLabel, SystemSet};
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
    physics_events::physics_events,
    resources::Airlock,
};

use super::{
    airlock_added::{airlock_added, airlock_default_map_added},
    airlock_events::airlock_events,
    airlock_tick_timers::airlock_tick_timers,
    entity_update::airlock_update,
    spawn::{build_airlocks, AirlockType},
};
use bevy::app::CoreStage::PostUpdate;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
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
                .add_system(airlock_added)
                .add_system(airlock_tick_timers)
                .add_system(airlock_events)
                .add_system(airlock_default_map_added)
                .add_system(physics_events)
                .add_event::<AirlockLockClosed>()
                .add_event::<AirlockUnlock>()
                .add_system_set_to_stage(
                    PostUpdate,
                    SystemSet::new()
                        .label(PostUpdateLabels::EntityUpdate)
                        .with_system(airlock_update),
                )
                .add_system(
                    health_combat_hit_result_sfx::<Airlock>
                        .after(CombatLabels::FinalizeApplyDamage),
                )
                .add_system(
                    toggle_open_action_prequisite_check
                        .label(ActionsLabels::Approve)
                        .after(ActionsLabels::Build),
                )
                .add_system(
                    lock_action_prequisite_check
                        .label(ActionsLabels::Approve)
                        .after(ActionsLabels::Build),
                )
                .add_system(
                    airlock_actions
                        .label(ActionsLabels::Action)
                        .after(ActionsLabels::Approve),
                )
                .add_system(
                    build_actions
                        .label(ActionsLabels::Build)
                        .after(ActionsLabels::Init),
                );
        }
        app.add_system(build_airlocks::<AirlockType>.after(BuildingLabels::TriggerBuild))
            .add_system((build_rigid_bodies::<AirlockType>).after(BuildingLabels::TriggerBuild))
            .add_system((build_base_entities::<AirlockType>).after(BuildingLabels::TriggerBuild));
        register_entity_type::<AirlockType>(app);
    }
}
