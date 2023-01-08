use bevy::prelude::{Entity, EventWriter, Query, Transform};

use math::grid::world_to_cell_id;

use crate::{
    sensable::Sensable,
    senser::{to_doryen_coordinates, Senser},
};
use networking::server::ConnectedPlayer;

use crate::spawning_events::SpawnClientEntity;
use networking::server::OutgoingReliableServerMessage;

use crate::net::EntityServerMessage;
/// Perform FOV checks to see what is and what isn't visible.

pub(crate) fn visible_checker(
    mut query_visible_entities: Query<(Entity, &mut Sensable, &Transform)>,
    mut query_visible_checker_entities_rigid: Query<(
        Entity,
        &mut Senser,
        &Transform,
        Option<&ConnectedPlayer>,
    )>,
    mut load_entity_event: EventWriter<SpawnClientEntity>,
    mut server: EventWriter<OutgoingReliableServerMessage<EntityServerMessage>>,
) {
    for (
        visible_checker_entity_id,
        mut senser_component,
        visible_checker_rigid_body_position_component,
        visible_checker_component_option,
    ) in query_visible_checker_entities_rigid.iter_mut()
    {
        let visible_checker_translation = visible_checker_rigid_body_position_component.translation;

        for (visible_entity_id, mut sensable_component, visible_transform_component) in
            query_visible_entities.iter_mut()
        {
            let visible_entity_transform;

            visible_entity_transform = visible_transform_component;

            let distance =
                visible_checker_translation.distance(visible_entity_transform.translation);

            let mut is_sensed = false;

            if sensable_component.is_light == false
                && sensable_component.is_audible == false
                && sensable_component.always_sensed == false
            {
                let visible_entity_cell_id = world_to_cell_id(visible_entity_transform.translation);
                let coords =
                    to_doryen_coordinates(visible_entity_cell_id.x, visible_entity_cell_id.z);
                is_sensed = senser_component.fov.is_in_fov(coords.0, coords.1);
            }

            if sensable_component.is_light {
                is_sensed = distance < LIGHT_DISTANCE;
            } else if sensable_component.is_audible {
                is_sensed = distance < HEAR_DISTANCE;
            }

            if sensable_component.always_sensed == true
                || visible_checker_entity_id == visible_entity_id
            {
                is_sensed = true;
            }

            let sensed_by_contains = sensable_component
                .sensed_by
                .contains(&visible_checker_entity_id);

            if is_sensed == false {
                if sensed_by_contains {
                    match visible_checker_component_option {
                        Some(visible_checker_component) => {
                            if visible_checker_component.connected {
                                server.send(OutgoingReliableServerMessage {
                                    handle: visible_checker_component.handle,
                                    message: EntityServerMessage::UnloadEntity(
                                        visible_entity_id.to_bits(),
                                    ),
                                });
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
                                load_entity_event.send(SpawnClientEntity {
                                    entity: visible_entity_id,
                                    loader_handle: visible_checker_component.handle,
                                });
                            }
                        }
                        None => {}
                    }
                }
            }
        }

        let mut gone_entities = vec![];
        let mut gone_sfx_entities = vec![];

        let mut i = 0;
        for entity in senser_component.sensing.iter() {
            match query_visible_entities.get(*entity) {
                Ok(_) => {}
                Err(_) => {
                    // Entity has despawned.
                    if !senser_component.sfx.contains(entity) {
                        match visible_checker_component_option {
                            Some(connected_component) => {
                                server.send(OutgoingReliableServerMessage {
                                    handle: connected_component.handle,
                                    message: EntityServerMessage::UnloadEntity(entity.to_bits()),
                                });
                            }
                            None => {}
                        }
                    } else {
                        let index = senser_component
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
            senser_component.sensing.remove(i);
        }

        for i in gone_sfx_entities {
            senser_component.sfx.remove(i);
        }
    }
}

const HEAR_DISTANCE: f32 = 60.;

const LIGHT_DISTANCE: f32 = 60.;
