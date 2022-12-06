use bevy::prelude::ResMut;

use bevy::prelude::warn;
use bevy::prelude::Vec3;
use bevy_renet::renet::RenetServer;
use networking::plugin::RENET_RELIABLE_CHANNEL_ID;
use networking::server::GridMapLayer;
use serde::Deserialize;
use serde::Serialize;
use typename::TypeName;

use crate::examine::InputExamineMap;
use bevy::prelude::EventWriter;
use bevy::prelude::Res;
use math::grid::Vec3Int;
use networking::server::HandleToEntity;

/// Gets serialized and sent over the net, this is the client message.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum GridmapClientMessage {
    ExamineMap(GridMapLayer, i16, i16, i16),
}

/// Manage incoming network messages from clients.
#[cfg(feature = "server")]
pub(crate) fn incoming_messages(
    mut server: ResMut<RenetServer>,
    handle_to_entity: Res<HandleToEntity>,
    mut input_examine_map: EventWriter<InputExamineMap>,
) {
    for handle in server.clients_id().into_iter() {
        while let Some(message) = server.receive_message(handle, RENET_RELIABLE_CHANNEL_ID) {
            let client_message_result: Result<GridmapClientMessage, _> =
                bincode::deserialize(&message);
            let client_message;
            match client_message_result {
                Ok(x) => {
                    client_message = x;
                }
                Err(_rr) => {
                    continue;
                }
            }

            match client_message {
                GridmapClientMessage::ExamineMap(
                    grid_map_type,
                    cell_id_x,
                    cell_id_y,
                    cell_id_z,
                ) => match handle_to_entity.map.get(&handle) {
                    Some(player_entity) => {
                        input_examine_map.send(InputExamineMap {
                            handle: handle,
                            entity: *player_entity,
                            gridmap_type: grid_map_type,
                            gridmap_cell_id: Vec3Int {
                                x: cell_id_x,
                                y: cell_id_y,
                                z: cell_id_z,
                            },
                            ..Default::default()
                        });
                    }
                    None => {
                        warn!("Couldn't find player_entity belonging to ExamineMap sender handle.");
                    }
                },
            }
        }
    }
}
/// Gets serialized and sent over the net, this is the server message.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum GridmapServerMessage {
    RemoveCell(i16, i16, i16, GridMapLayer),
    AddCell(i16, i16, i16, i64, i64, GridMapLayer),
    FireProjectile(ProjectileData),
    ConfigBlackCellID(i64, i64),
    ConfigOrderedCellsMain(Vec<String>),
    ConfigOrderedCellsDetails1(Vec<String>),
    ConfigPlaceableItemsSurfaces(Vec<i64>),
    ConfigNonBlockingCells(Vec<i64>),
}

/// Contains information about the projectile and its visual graphics.
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum ProjectileData {
    Laser((f32, f32, f32, f32), f32, f32, Vec3, Vec3),
    Ballistic,
}
