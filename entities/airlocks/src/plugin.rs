use bevy::prelude::{App, IntoSystemDescriptor, Plugin, ResMut, SystemLabel, SystemSet, Transform};
use combat::sfx::health_combat_hit_result_sfx;
use entity::meta::GridItemData;
use entity::register::register_entity_type;
use entity::{
    entity_data::initialize_entity_data,
    meta::{EntityDataProperties, EntityDataResource},
    spawn::build_base_entities,
};
use physics::spawn::build_rigid_bodies;
use resources::is_server::is_server;
use resources::labels::{
    ActionsLabels, BuildingLabels, CombatLabels, PostUpdateLabels, StartupLabels,
};

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
    spawn::{
        build_airlocks, build_raw_airlocks, default_build_airlocks, AirlockType,
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
        app.add_startup_system(content_initialization.before(StartupLabels::BuildGridmap))
            .add_system(build_airlocks::<AirlockType>.after(BuildingLabels::TriggerBuild))
            .add_system((build_rigid_bodies::<AirlockType>).after(BuildingLabels::TriggerBuild))
            .add_system((build_base_entities::<AirlockType>).after(BuildingLabels::TriggerBuild))
            .add_system((build_raw_airlocks).after(BuildingLabels::TriggerBuild))
            .add_system(
                default_build_airlocks
                    .label(BuildingLabels::DefaultBuild)
                    .after(BuildingLabels::NormalBuild),
            );
        register_entity_type::<AirlockType>(app);
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
