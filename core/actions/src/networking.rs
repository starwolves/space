use bevy::prelude::ResMut;

use crate::core::InputAction;
use crate::core::InputListActionsEntity;
use crate::core::InputListActionsMap;
use bevy::prelude::warn;
use bevy::prelude::Entity;
use bevy_renet::renet::RenetServer;
use math::grid::Vec3Int;
use networking::plugin::RENET_RELIABLE_CHANNEL_ID;
use networking::server::HandleToEntity;
use networking::server::ReliableClientMessage;

use bevy::prelude::{EventWriter, Res};
/// Manage incoming network messages from clients.
#[cfg(feature = "server")]
pub(crate) fn incoming_messages(
    mut server: ResMut<RenetServer>,
    handle_to_entity: Res<HandleToEntity>,
    mut action_data_entity: EventWriter<InputListActionsEntity>,
    mut action_data_map: EventWriter<InputListActionsMap>,
    mut input_action: EventWriter<InputAction>,
) {
    for handle in server.clients_id().into_iter() {
        while let Some(message) = server.receive_message(handle, RENET_RELIABLE_CHANNEL_ID) {
            let client_message_result: Result<ReliableClientMessage, _> =
                bincode::deserialize(&message);
            let client_message;
            match client_message_result {
                Ok(x) => {
                    client_message = x;
                }
                Err(_rr) => {
                    warn!("Received invalid client message.");
                    continue;
                }
            }

            match client_message {
                ReliableClientMessage::TabDataEntity(entity_id_bits) => {
                    match handle_to_entity.map.get(&handle) {
                        Some(player_entity) => {
                            action_data_entity.send(InputListActionsEntity {
                                requested_by_entity: *player_entity,
                                targetted_entity: Entity::from_bits(entity_id_bits),
                                with_ui: true,
                            });
                        }
                        None => {
                            warn!("Couldn't find player_entity belonging to TabDataEntity sender handle.");
                        }
                    }
                }

                ReliableClientMessage::TabDataMap(gridmap_type, idx, idy, idz) => {
                    match handle_to_entity.map.get(&handle) {
                        Some(player_entity) => {
                            action_data_map.send(InputListActionsMap {
                                requested_by_entity: *player_entity,
                                gridmap_type: gridmap_type,
                                gridmap_cell_id: Vec3Int {
                                    x: idx,
                                    y: idy,
                                    z: idz,
                                },
                                with_ui: true,
                            });
                        }
                        None => {
                            warn!("Couldn't find player_entity belonging to ExamineMap sender handle.");
                        }
                    }
                }

                ReliableClientMessage::TabPressed(
                    id,
                    entity_option,
                    cell_option,
                    belonging_entity,
                ) => {
                    let mut entity_p_op = None;
                    match entity_option {
                        Some(s) => {
                            entity_p_op = Some(Entity::from_bits(s));
                        }
                        None => {}
                    }
                    let entity_b_op;
                    match belonging_entity {
                        Some(s) => {
                            entity_b_op = Entity::from_bits(s);
                        }
                        None => {
                            warn!("no examiner entity passed.");
                            continue;
                        }
                    }

                    let mut cell_option_op = None;

                    match cell_option {
                        Some(c) => {
                            cell_option_op = Some((
                                c.0,
                                Vec3Int {
                                    x: c.1,
                                    y: c.2,
                                    z: c.3,
                                },
                            ));
                        }
                        None => {}
                    }

                    input_action.send(InputAction {
                        fired_action_id: id,
                        target_entity_option: entity_p_op,
                        target_cell_option: cell_option_op,
                        action_taker: entity_b_op,
                    });
                }
                _ => (),
            }
        }
    }
}
