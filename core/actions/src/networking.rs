use networking::server::GridMapLayer;
use serde::Deserialize;
use serde::Serialize;

use crate::core::InputAction;
use crate::core::InputListActionsEntity;
use crate::core::InputListActionsMap;
use bevy::prelude::warn;
use bevy::prelude::Entity;
use math::grid::Vec3Int;
use networking::server::HandleToEntity;

use crate::net::ActionsClientMessage;
use bevy::prelude::{EventWriter, Res};

use bevy::prelude::EventReader;
use networking::server::IncomingReliableClientMessage;

/// Manage incoming network messages from clients.

pub(crate) fn incoming_messages(
    mut server: EventReader<IncomingReliableClientMessage<ActionsClientMessage>>,
    handle_to_entity: Res<HandleToEntity>,
    mut action_data_entity: EventWriter<InputListActionsEntity>,
    mut action_data_map: EventWriter<InputListActionsMap>,
    mut input_action: EventWriter<InputAction>,
) {
    for message in server.iter() {
        let client_message = message.message.clone();
        match client_message {
            ActionsClientMessage::TabDataEntity(entity_id_bits) => {
                match handle_to_entity.map.get(&message.handle) {
                    Some(player_entity) => {
                        action_data_entity.send(InputListActionsEntity {
                            requested_by_entity: *player_entity,
                            targetted_entity: Entity::from_bits(entity_id_bits),
                            with_ui: true,
                        });
                    }
                    None => {
                        warn!(
                            "Couldn't find player_entity belonging to TabDataEntity sender handle."
                        );
                    }
                }
            }

            ActionsClientMessage::TabDataMap(gridmap_type, idx, idy, idz) => {
                match handle_to_entity.map.get(&message.handle) {
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

            ActionsClientMessage::TabPressed(id, entity_option, cell_option, belonging_entity) => {
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
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct NetAction {
    pub id: String,
    pub text: String,
    pub tab_list_priority: u8,
    pub item_name: String,
    pub entity_option: Option<u64>,
    pub belonging_entity: Option<u64>,
    pub cell_option: Option<(GridMapLayer, i16, i16, i16)>,
}
