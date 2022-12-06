use crate::grid::GridmapData;
use bevy::prelude::{EventReader, Res};
use player::connection::SendServerConfiguration;

use crate::networking::GridmapServerMessage;

use bevy::prelude::EventWriter;
use networking::typenames::OutgoingReliableServerMessage;
#[cfg(feature = "server")]
pub(crate) fn configure(
    mut config_events: EventReader<SendServerConfiguration>,
    mut server: EventWriter<OutgoingReliableServerMessage<GridmapServerMessage>>,
    gridmap_data: Res<GridmapData>,
) {
    for event in config_events.iter() {
        server.send(OutgoingReliableServerMessage {
            handle: event.handle,
            message: GridmapServerMessage::ConfigBlackCellID(
                gridmap_data.blackcell_id,
                gridmap_data.blackcell_blocking_id,
            ),
        });

        server.send(OutgoingReliableServerMessage {
            handle: event.handle,
            message: GridmapServerMessage::ConfigOrderedCellsMain(
                gridmap_data.ordered_main_names.clone(),
            ),
        });

        server.send(OutgoingReliableServerMessage {
            handle: event.handle,
            message: GridmapServerMessage::ConfigOrderedCellsDetails1(
                gridmap_data.ordered_details1_names.clone(),
            ),
        });

        server.send(OutgoingReliableServerMessage {
            handle: event.handle,
            message: GridmapServerMessage::ConfigPlaceableItemsSurfaces(
                gridmap_data.placeable_items_cells_list.clone(),
            ),
        });

        server.send(OutgoingReliableServerMessage {
            handle: event.handle,
            message: GridmapServerMessage::ConfigNonBlockingCells(
                gridmap_data.non_fov_blocking_cells_list.clone(),
            ),
        });
    }
}
