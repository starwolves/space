use bevy_ecs::{change_detection::Mut, entity::Entity, event::EventWriter, system::Query};
use bevy_math::Vec3;
use bevy_transform::components::Transform;

use crate::core::{
    connected_player::components::ConnectedPlayer,
    entity::{
        components::{EntityData, EntityUpdates},
        events::{NetLoadEntity, NetUnloadEntity},
        functions::{load_entity_for_player::load_entity, unload_entity_for_player::unload_entity},
    },
    gridmap::{functions::gridmap_functions::world_to_cell_id, resources::to_doryen_coordinates},
    physics::components::{WorldMode, WorldModes},
    sensable::components::Sensable,
    senser::components::Senser,
    static_body::components::StaticTransform,
};

pub fn visible_checker(
    mut query_visible_entities: Query<(
        Entity,
        &mut Sensable,
        Option<&StaticTransform>,
        Option<&Transform>,
        &EntityData,
        &EntityUpdates,
        Option<&WorldMode>,
    )>,
    mut query_visible_checker_entities_rigid: Query<(
        Entity,
        &mut Senser,
        &Transform,
        Option<&ConnectedPlayer>,
    )>,
    mut net_load_entity: EventWriter<NetLoadEntity>,
    mut net_unload_entity: EventWriter<NetUnloadEntity>,
) {
    for (
        entity,
        mut visible_checker_component,
        visible_checker_rigid_body_position_component,
        visible_checker_connected_player_component_option,
    ) in query_visible_checker_entities_rigid.iter_mut()
    {
        let visible_checker_translation = visible_checker_rigid_body_position_component.translation;

        let visible_checker_translation_vec = Vec3::new(
            visible_checker_translation.x,
            visible_checker_translation.y,
            visible_checker_translation.z,
        );

        for (
            visible_entity_id,
            mut visible_component,
            static_transform_component_option,
            rigid_body_position_component_option,
            entity_data_component,
            entity_updates_component,
            entity_world_mode_option,
        ) in query_visible_entities.iter_mut()
        {
            let visible_entity_transform;

            let mut is_interpolated = false;

            match static_transform_component_option {
                Some(static_transform) => {
                    visible_entity_transform = static_transform.transform;
                }
                None => {
                    match entity_world_mode_option {
                        Some(entity_world_mode) => {
                            if matches!(entity_world_mode.mode, WorldModes::Held)
                                || matches!(entity_world_mode.mode, WorldModes::Worn)
                            {
                                is_interpolated = false;
                            } else {
                                is_interpolated = true;
                            }
                        }
                        None => {
                            is_interpolated = false;
                        }
                    }

                    let visible_entity_isometry = rigid_body_position_component_option.unwrap();

                    visible_entity_transform = *visible_entity_isometry;
                }
            }

            visible_check(
                &mut visible_component,
                &mut visible_checker_component,
                visible_entity_transform,
                visible_checker_translation_vec,
                entity,
                &mut net_load_entity,
                &mut net_unload_entity,
                visible_checker_connected_player_component_option,
                entity_data_component,
                visible_entity_id,
                is_interpolated,
                &entity_updates_component,
            );
        }

        let mut gone_entities = vec![];
        let mut gone_sfx_entities = vec![];

        let mut i = 0;
        for entity in visible_checker_component.sensing.iter() {
            match query_visible_entities.get(*entity) {
                Ok(_) => {}
                Err(_) => {
                    // Entity has despawned.
                    if !visible_checker_component.sfx.contains(entity) {
                        match visible_checker_connected_player_component_option {
                            Some(connected_component) => {
                                unload_entity(
                                    connected_component.handle,
                                    *entity,
                                    &mut net_unload_entity,
                                    true,
                                );
                            }
                            None => {}
                        }
                    } else {
                        let index = visible_checker_component
                            .sfx
                            .iter()
                            .position(|&r| r == *entity)
                            .unwrap();
                        gone_sfx_entities.push(index);
                    }

                    gone_entities.push(i);
                }
            }

            i += 1;
        }

        gone_entities.reverse();
        gone_sfx_entities.reverse();

        for i in gone_entities {
            visible_checker_component.sensing.remove(i);
        }

        for i in gone_sfx_entities {
            visible_checker_component.sfx.remove(i);
        }
    }
}

const VIEW_DISTANCE: f32 = 90.;
const HEAR_DISTANCE: f32 = 60.;
const LIGHT_DISTANCE: f32 = 180.;

fn visible_check(
    sensable_component: &mut Mut<Sensable>,
    senser_component: &mut Mut<Senser>,
    visible_entity_transform: Transform,
    visible_checker_translation: Vec3,
    visible_checker_entity_id: Entity,
    net_load_entity: &mut EventWriter<NetLoadEntity>,
    net_unload_entity: &mut EventWriter<NetUnloadEntity>,
    visible_checker_component_option: Option<&ConnectedPlayer>,
    visible_entity_data: &EntityData,
    visible_entity_id: Entity,
    interpolated_transform: bool,
    visible_entity_updates_component: &EntityUpdates,
) {
    let distance = visible_checker_translation.distance(visible_entity_transform.translation);
    let is_cached = distance < VIEW_DISTANCE;
    let can_cache;

    if sensable_component.is_light
        || sensable_component.is_audible
        || sensable_component.always_sensed
    {
        can_cache = false;
    } else {
        can_cache = true;
    }

    let mut is_sensed = false;

    if sensable_component.is_light == false
        && sensable_component.is_audible == false
        && sensable_component.always_sensed == false
        && is_cached
    {
        let visible_entity_cell_id = world_to_cell_id(visible_entity_transform.translation);
        let coords = to_doryen_coordinates(visible_entity_cell_id.x, visible_entity_cell_id.z);
        is_sensed = senser_component.fov.is_in_fov(coords.0, coords.1);
    }

    if sensable_component.is_light {
        is_sensed = distance < LIGHT_DISTANCE;
    } else if sensable_component.is_audible {
        is_sensed = distance < HEAR_DISTANCE;
    }

    if sensable_component.always_sensed == true || visible_checker_entity_id == visible_entity_id {
        is_sensed = true;
    }

    let sensed_by_contains = sensable_component
        .sensed_by
        .contains(&visible_checker_entity_id);
    let sensed_by_cached_contains = sensable_component
        .sensed_by_cached
        .contains(&visible_checker_entity_id);

    if is_sensed == false {
        let unload_entirely;

        if can_cache {
            unload_entirely = !is_cached;
        } else {
            unload_entirely = true;
        }

        if sensed_by_contains {
            match visible_checker_component_option {
                Some(visible_checker_component) => {
                    if visible_checker_component.connected {
                        unload_entity(
                            visible_checker_component.handle,
                            visible_entity_id,
                            net_unload_entity,
                            unload_entirely,
                        );
                    }
                }
                None => {}
            }

            let index1 = sensable_component
                .sensed_by
                .iter()
                .position(|x| x == &visible_checker_entity_id)
                .unwrap();
            sensable_component.sensed_by.remove(index1);

            match senser_component
                .sensing
                .iter()
                .position(|x| x == &visible_checker_entity_id)
            {
                Some(index) => {
                    senser_component.sensing.remove(index);
                }
                None => {}
            }

            if can_cache && !unload_entirely {
                if !sensed_by_cached_contains {
                    sensable_component
                        .sensed_by_cached
                        .push(visible_checker_entity_id);
                }
            }
        } else if sensed_by_cached_contains && unload_entirely {
            match visible_checker_component_option {
                Some(visible_checker_component) => {
                    if visible_checker_component.connected {
                        unload_entity(
                            visible_checker_component.handle,
                            visible_entity_id,
                            net_unload_entity,
                            unload_entirely,
                        );
                    }
                }
                None => {}
            }

            let index = sensable_component
                .sensed_by_cached
                .iter()
                .position(|x| x == &visible_checker_entity_id)
                .unwrap();
            sensable_component.sensed_by_cached.remove(index);

            match senser_component
                .sensing
                .iter()
                .position(|x| x == &visible_checker_entity_id)
            {
                Some(index) => {
                    senser_component.sensing.remove(index);
                }
                None => {}
            }
        } else if !sensed_by_contains && !sensed_by_cached_contains {
            if can_cache && !unload_entirely {
                match visible_checker_component_option {
                    Some(visible_checker_component) => {
                        if visible_checker_component.connected {
                            unload_entity(
                                visible_checker_component.handle,
                                visible_entity_id,
                                net_unload_entity,
                                unload_entirely,
                            );
                        }
                    }
                    None => {}
                }

                sensable_component
                    .sensed_by_cached
                    .push(visible_checker_entity_id);
            }
        }
    } else {
        if !senser_component.sensing.contains(&visible_entity_id) {
            senser_component.sensing.push(visible_entity_id);
            if sensable_component.is_audible {
                senser_component.sfx.push(visible_entity_id);
            }
        }

        if !sensed_by_contains {
            sensable_component.sensed_by.push(visible_checker_entity_id);

            match visible_checker_component_option {
                Some(visible_checker_component) => {
                    if visible_checker_component.connected {
                        load_entity(
                            &visible_entity_updates_component.updates,
                            visible_entity_transform,
                            interpolated_transform,
                            net_load_entity,
                            visible_checker_component.handle,
                            visible_entity_data,
                            visible_entity_updates_component,
                            visible_entity_id,
                            true,
                        );
                    }
                }
                None => {}
            }
        }

        if sensed_by_cached_contains {
            let index = sensable_component
                .sensed_by_cached
                .iter()
                .position(|x| x == &visible_checker_entity_id)
                .unwrap();
            sensable_component.sensed_by_cached.remove(index);
        }
    }
}
