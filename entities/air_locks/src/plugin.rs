use bevy::prelude::{
    App, ParallelSystemDescriptorCoercion, Plugin, ResMut, SystemLabel, SystemSet, Transform,
};
use entity::{
    entity_data::initialize_entity_data,
    spawn::{summon_base_entity, SpawnEvent},
};
use networking::messages::net_system;
use rigid_body::spawn::summon_rigid_body;
use api::{
    data::{
        EntityDataProperties, EntityDataResource, PostUpdateLabels, StartupLabels, SummoningLabels,
    },
    gridmap::GridItemData,
    tab_actions::TabActionsQueueLabels,
};

use crate::physics_events::physics_events;

use super::{
    actions::air_locks_actions,
    air_lock_added::{
        air_lock_added, air_lock_default_map_added, AirLockCollision, AirLockLockClosed,
        AirLockLockOpen, AirLockUnlock, InputAirLockToggleOpen,
    },
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
            .add_system(air_locks_actions.after(TabActionsQueueLabels::TabAction))
            .add_system_set_to_stage(
                PostUpdate,
                SystemSet::new()
                    .after(PostUpdateLabels::VisibleChecker)
                    .label(PostUpdateLabels::Net)
                    .with_system(net_system::<NetAirLock>),
            )
            .add_system(summon_air_lock::<AirlockSummoner>.after(SummoningLabels::TriggerSummon))
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
            );
    }
}

pub fn content_initialization(mut entity_data: ResMut<EntityDataResource>) {
    let entity_properties = EntityDataProperties {
        name: SECURITY_AIRLOCK_ENTITY_NAME.to_string(),
        id: entity_data.get_id_inc(),
        grid_item: Some(GridItemData {
            transform_offset: Transform::identity(),
            can_be_built_with_grid_item: vec![],
        }),
    };

    initialize_entity_data(&mut entity_data, entity_properties);

    let entity_properties = EntityDataProperties {
        name: VACUUM_AIRLOCK_ENTITY_NAME.to_string(),
        id: entity_data.get_id_inc(),
        grid_item: Some(GridItemData {
            transform_offset: Transform::identity(),
            can_be_built_with_grid_item: vec![],
        }),
    };

    initialize_entity_data(&mut entity_data, entity_properties);

    let entity_properties = EntityDataProperties {
        name: GOVERNMENT_AIRLOCK_ENTITY_NAME.to_string(),
        id: entity_data.get_id_inc(),
        grid_item: Some(GridItemData {
            transform_offset: Transform::identity(),
            can_be_built_with_grid_item: vec![],
        }),
    };

    initialize_entity_data(&mut entity_data, entity_properties);

    let entity_properties = EntityDataProperties {
        name: BRIDGE_AIRLOCK_ENTITY_NAME.to_string(),
        id: entity_data.get_id_inc(),
        grid_item: Some(GridItemData {
            transform_offset: Transform::identity(),
            can_be_built_with_grid_item: vec![],
        }),
    };

    initialize_entity_data(&mut entity_data, entity_properties);
}
