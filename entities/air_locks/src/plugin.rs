use std::env;

use bevy::prelude::{App, IntoSystemDescriptor, Plugin, ResMut, SystemLabel, SystemSet, Transform};
use combat::sfx::health_combat_hit_result_sfx;
use entity::{
    entity_data::initialize_entity_data,
    meta::{EntityDataProperties, EntityDataResource},
    spawn::{summon_base_entity, SpawnEvent},
};
use gridmap_meta::core::GridItemData;
use networking::server::net_system;
use resources::labels::{
    ActionsLabels, CombatLabels, PostUpdateLabels, StartupLabels, SummoningLabels,
};
use rigid_body::spawn::summon_rigid_body;

use crate::{
    actions::{
        air_lock_actions, build_actions, lock_action_prequisite_check,
        toggle_open_action_prequisite_check,
    },
    air_lock_events::{
        AirLockCollision, AirLockLockClosed, AirLockLockOpen, AirLockUnlock, InputAirLockToggleOpen,
    },
    physics_events::physics_events,
    resources::AirLock,
};

use super::{
    air_lock_added::{air_lock_added, air_lock_default_map_added},
    air_lock_events::air_lock_events,
    air_lock_tick_timers::air_lock_tick_timers,
    entity_update::air_lock_update,
    net::NetAirLock,
    spawn::{
        default_summon_air_lock, summon_air_lock, summon_raw_air_lock, AirlockSummoner,
        BRIDGE_AIRLOCK_ENTITY_NAME, GOVERNMENT_AIRLOCK_ENTITY_NAME, SECURITY_AIRLOCK_ENTITY_NAME,
        VACUUM_AIRLOCK_ENTITY_NAME,
    },
};
use bevy::app::CoreStage::PostUpdate;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum AirLockTimers {
    Timer,
}
pub struct AirLocksPlugin;

impl Plugin for AirLocksPlugin {
    fn build(&self, app: &mut App) {
        if env::var("CARGO_MANIFEST_DIR").unwrap().ends_with("server") {
            app.add_event::<AirLockCollision>()
                .add_event::<InputAirLockToggleOpen>()
                .add_event::<AirLockLockOpen>()
                .add_event::<NetAirLock>()
                .add_system(air_lock_added)
                .add_system(air_lock_tick_timers)
                .add_system(air_lock_events)
                .add_system(air_lock_default_map_added)
                .add_system(physics_events)
                .add_event::<AirLockLockClosed>()
                .add_event::<AirLockUnlock>()
                .add_event::<SpawnEvent<AirlockSummoner>>()
                .add_system_set_to_stage(
                    PostUpdate,
                    SystemSet::new()
                        .label(PostUpdateLabels::EntityUpdate)
                        .with_system(air_lock_update),
                )
                .add_startup_system(content_initialization.before(StartupLabels::BuildGridmap))
                .add_system_set_to_stage(
                    PostUpdate,
                    SystemSet::new()
                        .after(PostUpdateLabels::VisibleChecker)
                        .label(PostUpdateLabels::Net)
                        .with_system(net_system::<NetAirLock>),
                )
                .add_system(
                    summon_air_lock::<AirlockSummoner>.after(SummoningLabels::TriggerSummon),
                )
                .add_system(
                    (summon_rigid_body::<AirlockSummoner>).after(SummoningLabels::TriggerSummon),
                )
                .add_system(
                    (summon_base_entity::<AirlockSummoner>).after(SummoningLabels::TriggerSummon),
                )
                .add_system((summon_raw_air_lock).after(SummoningLabels::TriggerSummon))
                .add_system(
                    default_summon_air_lock
                        .label(SummoningLabels::DefaultSummon)
                        .after(SummoningLabels::NormalSummon),
                )
                .add_system(
                    health_combat_hit_result_sfx::<AirLock>
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
                    air_lock_actions
                        .label(ActionsLabels::Action)
                        .after(ActionsLabels::Approve),
                )
                .add_system(
                    build_actions
                        .label(ActionsLabels::Build)
                        .after(ActionsLabels::Init),
                );
        }
    }
}

pub fn content_initialization(mut entity_data: ResMut<EntityDataResource>) {
    let entity_properties = EntityDataProperties {
        name: SECURITY_AIRLOCK_ENTITY_NAME.to_string(),
        id: entity_data.get_id_inc(),
        grid_item: Some(GridItemData {
            transform_offset: Transform::IDENTITY,
            can_be_built_with_grid_item: vec![],
        }),
    };

    initialize_entity_data(&mut entity_data, entity_properties);

    let entity_properties = EntityDataProperties {
        name: VACUUM_AIRLOCK_ENTITY_NAME.to_string(),
        id: entity_data.get_id_inc(),
        grid_item: Some(GridItemData {
            transform_offset: Transform::IDENTITY,
            can_be_built_with_grid_item: vec![],
        }),
    };

    initialize_entity_data(&mut entity_data, entity_properties);

    let entity_properties = EntityDataProperties {
        name: GOVERNMENT_AIRLOCK_ENTITY_NAME.to_string(),
        id: entity_data.get_id_inc(),
        grid_item: Some(GridItemData {
            transform_offset: Transform::IDENTITY,
            can_be_built_with_grid_item: vec![],
        }),
    };

    initialize_entity_data(&mut entity_data, entity_properties);

    let entity_properties = EntityDataProperties {
        name: BRIDGE_AIRLOCK_ENTITY_NAME.to_string(),
        id: entity_data.get_id_inc(),
        grid_item: Some(GridItemData {
            transform_offset: Transform::IDENTITY,
            can_be_built_with_grid_item: vec![],
        }),
    };

    initialize_entity_data(&mut entity_data, entity_properties);
}
