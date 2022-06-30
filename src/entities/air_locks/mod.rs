use bevy_app::CoreStage::PostUpdate;
use bevy_app::{App, Plugin};
use bevy_ecs::schedule::{ParallelSystemDescriptorCoercion, SystemLabel, SystemSet};
use bevy_ecs::system::ResMut;
use bevy_transform::components::Transform;

use crate::core::entity::functions::initialize_entity_data::initialize_entity_data;
use crate::core::entity::resources::{EntityDataProperties, EntityDataResource, GridItemData};
use crate::core::entity::spawn::{summon_base_entity, SpawnEvent};
use crate::core::rigid_body::spawn::summon_rigid_body;
use crate::core::tab_actions::TabActionsQueueLabels;
use crate::core::{PostUpdateLabels, StartupLabels, SummoningLabels};

use self::events::{air_locks_actions, AirLockUnlock, NetAirLock};
use self::spawn::{
    default_summon_air_lock, summon_air_lock, summon_raw_air_lock, AirlockSummoner,
    BRIDGE_AIRLOCK_ENTITY_NAME, GOVERNMENT_AIRLOCK_ENTITY_NAME, SECURITY_AIRLOCK_ENTITY_NAME,
    VACUUM_AIRLOCK_ENTITY_NAME,
};
use self::systems::air_lock_added::air_lock_added;
use self::systems::air_lock_default_map_added::air_lock_default_map_added;
use self::systems::air_lock_events::air_lock_events;
use self::systems::air_lock_tick_timers::air_lock_tick_timers;
use self::systems::net_system::net_system;
use self::{
    entity_update::air_lock_update,
    events::{AirLockCollision, AirLockLockClosed, AirLockLockOpen, InputAirLockToggleOpen},
};

pub mod components;
pub mod entity_update;
pub mod events;
pub mod functions;
pub mod spawn;
pub mod systems;
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
            .add_system(air_lock_tick_timers.label(AirLockTimers::Timer))
            .add_system(air_lock_events.after(AirLockTimers::Timer))
            .add_system(air_lock_default_map_added)
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
            .add_system_to_stage(
                PostUpdate,
                net_system
                    .after(PostUpdateLabels::VisibleChecker)
                    .label(PostUpdateLabels::Net),
            )
            .add_system(summon_air_lock::<AirlockSummoner>.after(SummoningLabels::TriggerSummon))
            .add_system(
                (summon_rigid_body::<AirlockSummoner>).after(SummoningLabels::TriggerSummon),
            )
            .add_system(
                (summon_base_entity::<AirlockSummoner>).after(SummoningLabels::TriggerSummon),
            )
            .add_system((summon_raw_air_lock).after(SummoningLabels::TriggerSummon))
            .add_system(default_summon_air_lock.after(SummoningLabels::DefaultSummon));
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
