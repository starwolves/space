use std::collections::BTreeMap;

use bevy_app::EventReader;
use bevy_core::{Time, Timer};
use bevy_ecs::{
    entity::Entity,
    prelude::Added,
    system::{Commands, Query, Res, ResMut},
};
use bevy_rapier3d::prelude::RigidBodyPositionComponent;

use crate::space::{
    core::{
        atmospherics::{functions::get_atmos_index, resources::AtmosphericsResource},
        chat::functions::{FURTHER_ITALIC_FONT, HEALTHY_COLOR},
        entity::components::{DefaultMapEntity, EntityData, EntityGroup},
        examinable::components::{Examinable, RichName},
        gridmap::{
            functions::gridmap_functions::world_to_cell_id,
            resources::{EntityGridData, GridmapMain, Vec2Int},
        },
        map::resources::{MapData, GREEN_MAP_TILE_COUNTER},
        pawn::components::{Pawn, SpaceAccess},
        sfx::{components::sfx_auto_destroy, resources::SfxAutoDestroyTimers},
        static_body::components::StaticTransform,
    },
    entities::{
        air_locks::systems::ToggleOpenRequest,
        sfx::counter_window::{
            counter_window_closed_sfx::CounterWindowClosedSfxBundle,
            counter_window_denied_sfx::CounterWindowDeniedSfxBundle,
            counter_window_open_sfx::CounterWindowOpenSfxBundle,
        },
    },
};

use super::{
    components::{
        CounterWindow, CounterWindowAccessLightsStatus, CounterWindowClosedTimer,
        CounterWindowDeniedTimer, CounterWindowOpenTimer, CounterWindowSensor, CounterWindowStatus,
    },
    events::{CounterWindowSensorCollision, InputCounterWindowToggleOpen},
};

pub fn counter_window_events(
    mut counter_window_sensor_collisions: EventReader<CounterWindowSensorCollision>,
    mut counter_window_toggle_open_action: EventReader<InputCounterWindowToggleOpen>,
    mut counter_window_query: Query<(
        &mut CounterWindow,
        &mut RigidBodyPositionComponent,
        &StaticTransform,
        Option<&mut CounterWindowOpenTimer>,
        Option<&mut CounterWindowDeniedTimer>,
        Option<&mut CounterWindowClosedTimer>,
        Entity,
    )>,
    counter_window_sensor_query: Query<&CounterWindowSensor>,
    pawn_query: Query<(&Pawn, &SpaceAccess)>,
    mut auto_destroy_timers: ResMut<SfxAutoDestroyTimers>,
    mut commands: Commands,
    mut atmospherics_resource: ResMut<AtmosphericsResource>,
) {
    for (
        mut counter_window_component,
        mut rigid_body_position_component,
        static_transform_component,
        counter_window_open_timer_option,
        counter_window_denied_timer_option,
        counter_window_closed_timer_option,
        counter_window_entity,
    ) in counter_window_query.iter_mut()
    {
        match counter_window_open_timer_option {
            Some(mut timer_component) => {
                if timer_component.timer.finished() == true {
                    timer_component.timer.pause();
                    timer_component.timer.reset();

                    counter_window_component.status = CounterWindowStatus::Closed;

                    let cell_id =
                        world_to_cell_id(rigid_body_position_component.position.translation.into());
                    let cell_id2 = Vec2Int {
                        x: cell_id.x,
                        y: cell_id.z,
                    };
                    if AtmosphericsResource::is_id_out_of_range(cell_id2) {
                        continue;
                    }
                    let atmos_id = get_atmos_index(cell_id2);
                    let atmospherics = atmospherics_resource
                        .atmospherics
                        .get_mut(atmos_id)
                        .unwrap();

                    atmospherics.blocked = true;
                    atmospherics.forces_push_up = false;

                    commands
                        .entity(counter_window_entity)
                        .insert(CounterWindowClosedTimer::default());
                }
            }
            None => {}
        }

        match counter_window_closed_timer_option {
            Some(mut timer_component) => {
                if timer_component.timer.finished() == true {
                    timer_component.timer.pause();
                    timer_component.timer.reset();

                    let mut counter_window_rigid_body_position =
                        rigid_body_position_component.position;

                    counter_window_rigid_body_position.translation.y -= 2.;

                    rigid_body_position_component.position = counter_window_rigid_body_position;

                    counter_window_component.access_lights =
                        CounterWindowAccessLightsStatus::Neutral;

                    let sfx_entity = commands
                        .spawn()
                        .insert_bundle(CounterWindowClosedSfxBundle::new(
                            static_transform_component.transform,
                        ))
                        .id();
                    sfx_auto_destroy(sfx_entity, &mut auto_destroy_timers);
                }
            }
            None => {}
        }

        match counter_window_denied_timer_option {
            Some(mut timer_component) => {
                if timer_component.timer.finished() == true {
                    timer_component.timer.pause();
                    timer_component.timer.reset();

                    counter_window_component.access_lights =
                        CounterWindowAccessLightsStatus::Neutral;
                }
            }
            None => {}
        }
    }

    let mut toggle_open_requests = vec![];

    for collision_event in counter_window_sensor_collisions.iter() {
        if collision_event.started == false {
            continue;
        }

        let counter_window_sensor_entity;
        let pawn_entity;

        if matches!(
            collision_event.collider1_group,
            EntityGroup::CounterWindowSensor
        ) {
            counter_window_sensor_entity = collision_event.collider1_entity;
            pawn_entity = collision_event.collider2_entity;
        } else {
            counter_window_sensor_entity = collision_event.collider2_entity;
            pawn_entity = collision_event.collider1_entity;
        }

        toggle_open_requests.push(ToggleOpenRequest {
            opener: pawn_entity,
            opened: counter_window_sensor_entity,
        });
    }

    for event in counter_window_toggle_open_action.iter() {
        toggle_open_requests.push(ToggleOpenRequest {
            opener: event.opener,
            opened: Entity::from_bits(event.opened),
        });
    }

    for request in toggle_open_requests {
        let pawn_space_access_component_result =
            pawn_query.get_component::<SpaceAccess>(request.opener);
        let pawn_space_access_component;

        match pawn_space_access_component_result {
            Ok(result) => {
                pawn_space_access_component = result;
            }
            Err(_err) => {
                continue;
            }
        }

        let counter_window_sensor_components_result =
            counter_window_sensor_query.get_component::<CounterWindowSensor>(request.opened);
        let counter_window_sensor_component;

        match counter_window_sensor_components_result {
            Ok(counter_window_sensor) => {
                counter_window_sensor_component = counter_window_sensor;
            }
            Err(_err) => {
                continue;
            }
        }

        let counter_window_components_result =
            counter_window_query.get_mut(counter_window_sensor_component.parent);

        let mut counter_window_component;
        let mut counter_window_rigid_body_position_component;
        let counter_window_static_transform_component;
        let counter_window_closed_timer_option;

        match counter_window_components_result {
            Ok(result) => {
                counter_window_component = result.0;
                counter_window_rigid_body_position_component = result.1;
                counter_window_static_transform_component = result.2;
                counter_window_closed_timer_option = result.5;
            }
            Err(_err) => {
                continue;
            }
        }

        let mut pawn_has_permission = false;

        for space_permission in &counter_window_component.access_permissions {
            if pawn_space_access_component
                .access
                .contains(space_permission)
                == true
            {
                pawn_has_permission = true;
                break;
            }
        }

        match counter_window_closed_timer_option {
            Some(mut counter_window_closed_timer) => {
                counter_window_closed_timer.timer.pause();
                counter_window_closed_timer.timer.reset();
            }
            None => {}
        }

        if pawn_has_permission == true {
            if !matches!(counter_window_component.status, CounterWindowStatus::Open) {
                let sfx_entity = commands
                    .spawn()
                    .insert_bundle(CounterWindowOpenSfxBundle::new(
                        counter_window_static_transform_component.transform,
                    ))
                    .id();
                sfx_auto_destroy(sfx_entity, &mut auto_destroy_timers);
            }

            let cell_id = world_to_cell_id(
                counter_window_rigid_body_position_component
                    .position
                    .translation
                    .into(),
            );
            let cell_id2 = Vec2Int {
                x: cell_id.x,
                y: cell_id.z,
            };
            if AtmosphericsResource::is_id_out_of_range(cell_id2) {
                continue;
            }
            let atmos_id = get_atmos_index(cell_id2);
            let atmospherics = atmospherics_resource
                .atmospherics
                .get_mut(atmos_id)
                .unwrap();

            atmospherics.blocked = false;
            atmospherics.forces_push_up = true;

            counter_window_component.status = CounterWindowStatus::Open;
            counter_window_component.access_lights = CounterWindowAccessLightsStatus::Granted;

            let mut counter_window_rigid_body_position =
                counter_window_rigid_body_position_component.position;

            counter_window_rigid_body_position.translation.y += 2.;

            counter_window_rigid_body_position_component.position =
                counter_window_rigid_body_position;

            commands
                .entity(counter_window_sensor_component.parent)
                .insert(CounterWindowOpenTimer::default());
        } else {
            counter_window_component.access_lights = CounterWindowAccessLightsStatus::Denied;

            commands
                .entity(counter_window_sensor_component.parent)
                .insert(CounterWindowDeniedTimer::default());

            let sfx_entity = commands
                .spawn()
                .insert_bundle(CounterWindowDeniedSfxBundle::new(
                    counter_window_static_transform_component.transform,
                ))
                .id();
            sfx_auto_destroy(sfx_entity, &mut auto_destroy_timers);
        }
    }
}

pub fn counter_window_tick_timers(
    time: Res<Time>,
    mut query_timer: Query<&mut Timer>,
    mut query_counter_window_open_timer: Query<&mut CounterWindowOpenTimer>,
    mut query_counter_window_denied_timer: Query<&mut CounterWindowDeniedTimer>,
    mut query_counter_window_closed_timer: Query<&mut CounterWindowClosedTimer>,

    mut sfx_auto_destroy_timers: ResMut<SfxAutoDestroyTimers>,
    mut commands: Commands,
) {
    for mut timer in query_timer.iter_mut() {
        timer.tick(time.delta());
    }
    for mut timer in query_counter_window_open_timer.iter_mut() {
        timer.timer.tick(time.delta());
    }
    for mut timer in query_counter_window_denied_timer.iter_mut() {
        timer.timer.tick(time.delta());
    }
    for mut timer in query_counter_window_closed_timer.iter_mut() {
        timer.timer.tick(time.delta());
    }

    let mut expired_sfx_entities: Vec<Entity> = vec![];

    for (sfx_entity, incremental) in &mut sfx_auto_destroy_timers.timers {
        *incremental += 1;
        if incremental >= &mut 2 {
            expired_sfx_entities.push(*sfx_entity);
        }
    }

    for i in 0..expired_sfx_entities.len() {
        let this_entity_id = expired_sfx_entities[i];

        let mut j = 0;
        for (sfx_entity, _timer) in &mut sfx_auto_destroy_timers.timers {
            if this_entity_id == *sfx_entity {
                break;
            }
            j += 1;
        }

        sfx_auto_destroy_timers.timers.remove(j);

        commands.entity(this_entity_id).despawn();
    }
}

pub fn counter_window_added(
    counter_windows: Query<(Entity, &RigidBodyPositionComponent), Added<CounterWindow>>,
    mut atmospherics_resource: ResMut<AtmosphericsResource>,
) {
    for (_airlock_entity, rigid_body_position_component) in counter_windows.iter() {
        let cell_id = world_to_cell_id(rigid_body_position_component.position.translation.into());
        let cell_id2 = Vec2Int {
            x: cell_id.x,
            y: cell_id.z,
        };
        if AtmosphericsResource::is_id_out_of_range(cell_id2) {
            continue;
        }
        let atmos_id = get_atmos_index(cell_id2);
        let atmospherics = atmospherics_resource
            .atmospherics
            .get_mut(atmos_id)
            .unwrap();

        atmospherics.blocked = true;
    }
}

pub fn counter_window_default_map_added(
    mut default_counter_windows: Query<
        (
            Entity,
            &RigidBodyPositionComponent,
            &DefaultMapEntity,
            &EntityData,
            &mut Examinable,
        ),
        Added<CounterWindow>,
    >,
    mut map_data: ResMut<MapData>,
    mut gridmap_main: ResMut<GridmapMain>,
) {
    for (
        counter_window_entity,
        rigid_body_position_component,
        _,
        entity_data_component,
        mut examinable_component,
    ) in default_counter_windows.iter_mut()
    {
        let cell_id = world_to_cell_id(rigid_body_position_component.position.translation.into());
        let cell_id2 = Vec2Int {
            x: cell_id.x,
            y: cell_id.z,
        };
        map_data.data.insert(cell_id2, GREEN_MAP_TILE_COUNTER);

        gridmap_main.entity_data.insert(
            cell_id,
            EntityGridData {
                entity: counter_window_entity,
                entity_name: entity_data_component.entity_name.to_string(),
            },
        );

        if entity_data_component.entity_name == "securityCounterWindow" {
            examinable_component.name = RichName {
                name: "security counter window".to_string(),
                n: false,
                ..Default::default()
            };
            let mut examine_map = BTreeMap::new();
            examine_map.insert(0, "An airtight security window. It will only grant access to those authorised to use it.".to_string());
            examine_map.insert(
                1,
                "[font=".to_string()
                    + FURTHER_ITALIC_FONT
                    + "][color="
                    + HEALTHY_COLOR
                    + "]It is fully operational.[/color][/font]",
            );
            examinable_component.assigned_texts = examine_map;
        } else if entity_data_component.entity_name == "bridgeCounterWindow" {
            examinable_component.name = RichName {
                name: "bridge counter window".to_string(),
                n: false,
                ..Default::default()
            };
            let mut examine_map = BTreeMap::new();
            examine_map.insert(0, "An airtight bridge window. It will only grant access to those authorised to use it.".to_string());
            examine_map.insert(
                1,
                "[font=".to_string()
                    + FURTHER_ITALIC_FONT
                    + "][color="
                    + HEALTHY_COLOR
                    + "]It is fully operational.[/color][/font]",
            );
            examinable_component.assigned_texts = examine_map;
        }
    }
}
