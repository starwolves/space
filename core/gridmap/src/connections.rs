use crate::grid::GridmapData;
use bevy::prelude::{EventReader, Res};
use player::connection::SendServerConfiguration;

use bevy::prelude::ResMut;
use bevy_renet::renet::RenetServer;

use crate::networking::GridmapServerMessage;
use networking::plugin::RENET_RELIABLE_CHANNEL_ID;

#[cfg(feature = "server")]
pub(crate) fn configure(
    mut config_events: EventReader<SendServerConfiguration>,
    mut server: ResMut<RenetServer>,
    gridmap_data: Res<GridmapData>,
) {
    for event in config_events.iter() {
        server.send_message(
            event.handle,
            RENET_RELIABLE_CHANNEL_ID,
            bincode::serialize(&GridmapServerMessage::ConfigBlackCellID(
                gridmap_data.blackcell_id,
                gridmap_data.blackcell_blocking_id,
            ))
            .unwrap(),
        );

        server.send_message(
            event.handle,
            RENET_RELIABLE_CHANNEL_ID,
            bincode::serialize(&GridmapServerMessage::ConfigOrderedCellsMain(
                gridmap_data.ordered_main_names.clone(),
            ))
            .unwrap(),
        );

        server.send_message(
            event.handle,
            RENET_RELIABLE_CHANNEL_ID,
            bincode::serialize(&GridmapServerMessage::ConfigOrderedCellsDetails1(
                gridmap_data.ordered_details1_names.clone(),
            ))
            .unwrap(),
        );

        server.send_message(
            event.handle,
            RENET_RELIABLE_CHANNEL_ID,
            bincode::serialize(&GridmapServerMessage::ConfigPlaceableItemsSurfaces(
                gridmap_data.placeable_items_cells_list.clone(),
            ))
            .unwrap(),
        );

        server.send_message(
            event.handle,
            RENET_RELIABLE_CHANNEL_ID,
            bincode::serialize(&GridmapServerMessage::ConfigNonBlockingCells(
                gridmap_data.non_fov_blocking_cells_list.clone(),
            ))
            .unwrap(),
        );
    }
}
