use bevy::{
    math::Quat,
    prelude::{App, IntoSystemDescriptor, Plugin, ResMut, SystemSet, Transform},
};
use combat::sfx::health_combat_hit_result_sfx;
use entity::{
    entity_data::initialize_entity_data,
    meta::{EntityDataProperties, EntityDataResource},
    spawn::build_base_entities,
};
use entity::{entity_types::init_entity_type, meta::GridItemData};
use physics::spawn::build_rigid_bodies;
use resources::{
    is_server::is_server,
    labels::{ActionsLabels, BuildingLabels, CombatLabels, PostUpdateLabels, StartupLabels},
};

use crate::{
    actions::{
        counter_window_actions, lock_open_action_prequisite_check,
        toggle_open_action_prequisite_check,
    },
    counter_window_events::CounterWindow,
    physics_events::physics_events,
};

use super::{
    counter_window_added::{counter_window_added, counter_window_default_map_added},
    counter_window_events::{
        counter_window_events, CounterWindowLockClosed, CounterWindowLockOpen,
        CounterWindowSensorCollision, CounterWindowUnlock, InputCounterWindowToggleOpen,
    },
    counter_window_tick_timers::counter_window_tick_timers,
    entity_update::counter_window_update,
    spawn::{
        build_counter_windows, build_raw_counter_windows, default_build_counter_windows,
        CounterWindowType, BRIDGE_COUNTER_WINDOW_ENTITY_NAME, SECURITY_COUNTER_WINDOW_ENTITY_NAME,
    },
};
use crate::actions::build_actions;
use bevy::app::CoreStage::PostUpdate;
pub struct CounterWindowsPlugin;

impl Plugin for CounterWindowsPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_event::<CounterWindowSensorCollision>()
                .add_system(counter_window_tick_timers)
                .add_system(counter_window_events)
                .add_system(counter_window_added)
                .add_system(counter_window_default_map_added)
                .add_system(physics_events)
                .add_event::<InputCounterWindowToggleOpen>()
                .add_event::<CounterWindowLockOpen>()
                .add_event::<CounterWindowLockClosed>()
                .add_event::<CounterWindowUnlock>()
                .add_system_set_to_stage(
                    PostUpdate,
                    SystemSet::new()
                        .label(PostUpdateLabels::EntityUpdate)
                        .with_system(counter_window_update),
                )
                .add_system(
                    health_combat_hit_result_sfx::<CounterWindow>
                        .after(CombatLabels::FinalizeApplyDamage),
                )
                .add_system(
                    toggle_open_action_prequisite_check
                        .label(ActionsLabels::Approve)
                        .after(ActionsLabels::Build),
                )
                .add_system(
                    lock_open_action_prequisite_check
                        .label(ActionsLabels::Approve)
                        .after(ActionsLabels::Build),
                )
                .add_system(
                    counter_window_actions
                        .label(ActionsLabels::Action)
                        .after(ActionsLabels::Approve),
                )
                .add_system(
                    build_actions
                        .label(ActionsLabels::Build)
                        .after(ActionsLabels::Init),
                );
        }
        init_entity_type::<CounterWindowType>(app);
        app.add_startup_system(content_initialization.before(StartupLabels::BuildGridmap))
            .add_system(
                build_counter_windows::<CounterWindowType>.after(BuildingLabels::TriggerBuild),
            )
            .add_system(
                (build_base_entities::<CounterWindowType>).after(BuildingLabels::TriggerBuild),
            )
            .add_system(
                (build_rigid_bodies::<CounterWindowType>).after(BuildingLabels::TriggerBuild),
            )
            .add_system((build_raw_counter_windows).after(BuildingLabels::TriggerBuild))
            .add_system(
                (default_build_counter_windows)
                    .label(BuildingLabels::DefaultBuild)
                    .after(BuildingLabels::NormalBuild),
            );
    }
}

pub fn content_initialization(mut entity_data: ResMut<EntityDataResource>) {
    let mut transform = Transform::IDENTITY;
    transform.translation.y = 0.86;
    transform.rotation = Quat::from_xyzw(0., 0.707, 0., 0.707);

    let entity_properties = EntityDataProperties {
        name: SECURITY_COUNTER_WINDOW_ENTITY_NAME.to_string(),
        id: entity_data.get_id_inc(),
        grid_item: Some(GridItemData {
            transform_offset: transform,
            can_be_built_with_grid_item: vec!["securityCounter1".to_string()],
        }),
    };

    initialize_entity_data(&mut entity_data, entity_properties);

    let mut transform = Transform::IDENTITY;
    transform.translation.y = 0.86;
    transform.rotation = Quat::from_xyzw(0., 0.707, 0., 0.707);

    let entity_properties = EntityDataProperties {
        name: BRIDGE_COUNTER_WINDOW_ENTITY_NAME.to_string(),
        id: entity_data.get_id_inc(),
        grid_item: Some(GridItemData {
            transform_offset: transform,
            can_be_built_with_grid_item: vec!["bridgeCounter".to_string()],
        }),
    };

    initialize_entity_data(&mut entity_data, entity_properties);
}
