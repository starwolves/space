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
        entity::components::{DefaultMapEntity, EntityData, EntityGroup},
        gridmap::{
            functions::gridmap_functions::world_to_cell_id,
            resources::{EntityGridData, GridmapMain, Vec2Int},
        },
        map::resources::{MapData, GREEN_MAP_TILE_ENTRANCE},
        pawn::components::{Pawn, SpaceAccess},
        sfx::{components::sfx_auto_destroy, resources::SfxAutoDestroyTimers},
        static_body::components::StaticTransform,
    },
    entities::{
        air_locks::components::{
            AccessLightsStatus, AirLock, AirLockClosedTimer, AirLockDeniedTimer, AirLockOpenTimer,
            AirLockStatus,
        },
        sfx::air_lock::{
            air_lock_closed_sfx::AirLockClosedSfxBundle,
            air_lock_denied_sfx::AirLockDeniedSfxBundle, air_lock_open_sfx::AirLockOpenSfxBundle,
        },
    },
};

use super::events::AirLockCollision;

pub fn air_lock_events(
    mut air_lock_collisions: EventReader<AirLockCollision>,
    mut air_lock_query: Query<(
        &mut AirLock,
        &mut RigidBodyPositionComponent,
        &StaticTransform,
        Option<&mut AirLockOpenTimer>,
        Option<&mut AirLockDeniedTimer>,
        Option<&mut AirLockClosedTimer>,
        Entity,
    )>,
    pawn_query: Query<(&Pawn, &SpaceAccess)>,
    mut auto_destroy_timers: ResMut<SfxAutoDestroyTimers>,
    mut commands: Commands,
    mut atmospherics_resource: ResMut<AtmosphericsResource>,
) {
    for (
        mut air_lock_component,
        mut rigid_body_position_component,
        static_transform_component,
        timer_open_component_option,
        timer_denied_component_option,
        timer_closed_component_option,
        air_lock_entity,
    ) in air_lock_query.iter_mut()
    {
        match timer_open_component_option {
            Some(mut timer_component) => {
                if timer_component.timer.finished() == true {
                    timer_component.timer.pause();
                    timer_component.timer.reset();

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
                    air_lock_component.status = AirLockStatus::Closed;

                    commands
                        .entity(air_lock_entity)
                        .insert(AirLockClosedTimer::default());
                }
            }
            None => {}
        }

        match timer_closed_component_option {
            Some(mut timer_component) => {
                if timer_component.timer.finished() == true {
                    timer_component.timer.pause();
                    timer_component.timer.reset();

                    let mut air_lock_rigid_body_position = rigid_body_position_component.position;

                    air_lock_rigid_body_position.translation.y -= 2.;

                    rigid_body_position_component.position = air_lock_rigid_body_position;

                    air_lock_component.access_lights = AccessLightsStatus::Neutral;

                    let sfx_entity = commands
                        .spawn()
                        .insert_bundle(AirLockClosedSfxBundle::new(
                            static_transform_component.transform,
                        ))
                        .id();
                    sfx_auto_destroy(sfx_entity, &mut auto_destroy_timers);
                }
            }
            None => {}
        }

        match timer_denied_component_option {
            Some(mut timer_component) => {
                if timer_component.timer.finished() == true {
                    timer_component.timer.pause();
                    timer_component.timer.reset();

                    air_lock_component.access_lights = AccessLightsStatus::Neutral;
                }
            }
            None => {}
        }
    }

    for collision_event in air_lock_collisions.iter() {
        if collision_event.started == false {
            continue;
        }

        let air_lock_entity;
        let pawn_entity;

        if matches!(collision_event.collider1_group, EntityGroup::AirLock) {
            air_lock_entity = collision_event.collider1_entity;
            pawn_entity = collision_event.collider2_entity;
        } else {
            air_lock_entity = collision_event.collider2_entity;
            pawn_entity = collision_event.collider1_entity;
        }

        let pawn_space_access_component_result =
            pawn_query.get_component::<SpaceAccess>(pawn_entity);
        let pawn_space_access_component;

        match pawn_space_access_component_result {
            Ok(result) => {
                pawn_space_access_component = result;
            }
            Err(_err) => {
                continue;
            }
        }

        let air_lock_components_result = air_lock_query.get_mut(air_lock_entity);

        let mut air_lock_component;
        let mut air_lock_rigid_body_position_component;
        let air_lock_static_transform_component;

        match air_lock_components_result {
            Ok(result) => {
                air_lock_component = result.0;
                air_lock_rigid_body_position_component = result.1;
                air_lock_static_transform_component = result.2;
            }
            Err(_err) => {
                continue;
            }
        }

        let mut pawn_has_permission = false;

        for space_permission in &air_lock_component.access_permissions {
            if pawn_space_access_component
                .access
                .contains(space_permission)
                == true
            {
                pawn_has_permission = true;
                break;
            }
        }

        if pawn_has_permission == true {
            let cell_id = world_to_cell_id(
                air_lock_rigid_body_position_component
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
            air_lock_component.status = AirLockStatus::Open;
            air_lock_component.access_lights = AccessLightsStatus::Granted;

            let mut air_lock_rigid_body_position = air_lock_rigid_body_position_component.position;

            air_lock_rigid_body_position.translation.y += 2.;

            air_lock_rigid_body_position_component.position = air_lock_rigid_body_position;

            commands
                .entity(air_lock_entity)
                .insert(AirLockOpenTimer::default());

            let sfx_entity = commands
                .spawn()
                .insert_bundle(AirLockOpenSfxBundle::new(
                    air_lock_static_transform_component.transform,
                ))
                .id();
            sfx_auto_destroy(sfx_entity, &mut auto_destroy_timers);
        } else {
            air_lock_component.access_lights = AccessLightsStatus::Denied;

            commands
                .entity(air_lock_entity)
                .insert(AirLockDeniedTimer::default());

            let sfx_entity = commands
                .spawn()
                .insert_bundle(AirLockDeniedSfxBundle::new(
                    air_lock_static_transform_component.transform,
                ))
                .id();
            sfx_auto_destroy(sfx_entity, &mut auto_destroy_timers);
        }
    }
}

pub fn air_lock_tick_timers(
    time: Res<Time>,
    mut query_timer: Query<&mut Timer>,
    mut query_air_lock_denied_timer: Query<&mut AirLockDeniedTimer>,
    mut query_air_lock_open_timer: Query<&mut AirLockOpenTimer>,
    mut query_air_lock_closed_timer: Query<&mut AirLockClosedTimer>,

    mut sfx_auto_destroy_timers: ResMut<SfxAutoDestroyTimers>,
    mut commands: Commands,
) {
    for mut timer in query_timer.iter_mut() {
        timer.tick(time.delta());
    }
    for mut timer in query_air_lock_denied_timer.iter_mut() {
        timer.timer.tick(time.delta());
    }
    for mut timer in query_air_lock_open_timer.iter_mut() {
        timer.timer.tick(time.delta());
    }
    for mut timer in query_air_lock_closed_timer.iter_mut() {
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

pub fn air_lock_added(
    air_locks: Query<(Entity, &RigidBodyPositionComponent), Added<AirLock>>,
    mut atmospherics_resource: ResMut<AtmosphericsResource>,
) {
    for (_airlock_entity, rigid_body_position_component) in air_locks.iter() {
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

pub fn air_lock_default_map_added(
    airlock_windows: Query<
        (
            Entity,
            &RigidBodyPositionComponent,
            &DefaultMapEntity,
            &EntityData,
        ),
        Added<AirLock>,
    >,
    mut map_data: ResMut<MapData>,
    mut gridmap_main: ResMut<GridmapMain>,
) {
    for (airlock_entity, rigid_body_position_component, _, entity_data_component) in
        airlock_windows.iter()
    {
        let cell_id = world_to_cell_id(rigid_body_position_component.position.translation.into());
        let cell_id2 = Vec2Int {
            x: cell_id.x,
            y: cell_id.z,
        };
        map_data.data.insert(cell_id2, GREEN_MAP_TILE_ENTRANCE);

        gridmap_main.entity_data.insert(
            cell_id,
            EntityGridData {
                entity: airlock_entity,
                entity_name: entity_data_component.entity_name.to_string(),
            },
        );
    }
}
