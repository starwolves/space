use crate::grid::GridmapData;
use bevy::prelude::{EventReader, EventWriter, Res};
use networking::server::PendingMessage;
use networking::server::PendingNetworkMessage;
use networking::server::{ReliableServerMessage, ServerConfigMessage};
use networking_macros::NetMessage;
use player::connection::SendServerConfiguration;

#[derive(NetMessage)]
#[cfg(feature = "server")]
pub(crate) struct NetConfig {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
#[cfg(feature = "server")]
pub(crate) fn configure(
    mut config_events: EventReader<SendServerConfiguration>,
    mut net_on_new_player_connection: EventWriter<NetConfig>,
    gridmap_data: Res<GridmapData>,
) {
    for event in config_events.iter() {
        net_on_new_player_connection.send(NetConfig {
            handle: event.handle,
            message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::BlackCellID(
                gridmap_data.blackcell_id,
                gridmap_data.blackcell_blocking_id,
            )),
        });

        net_on_new_player_connection.send(NetConfig {
            handle: event.handle,
            message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::OrderedCellsMain(
                gridmap_data.ordered_main_names.clone(),
            )),
        });

        net_on_new_player_connection.send(NetConfig {
            handle: event.handle,
            message: ReliableServerMessage::ConfigMessage(
                ServerConfigMessage::OrderedCellsDetails1(
                    gridmap_data.ordered_details1_names.clone(),
                ),
            ),
        });
        net_on_new_player_connection.send(NetConfig {
            handle: event.handle,
            message: ReliableServerMessage::ConfigMessage(
                ServerConfigMessage::PlaceableItemsSurfaces(
                    gridmap_data.placeable_items_cells_list.clone(),
                ),
            ),
        });

        net_on_new_player_connection.send(NetConfig {
            handle: event.handle,
            message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::NonBlockingCells(
                gridmap_data.non_fov_blocking_cells_list.clone(),
            )),
        });
    }
}
