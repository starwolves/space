use bevy::prelude::{Entity, Query, ResMut};

use entity::senser::{to_doryen_coordinates, Senser};

use crate::grid::Gridmap;

use networking::server::ConnectedPlayer;

use crate::net::GridmapServerMessage;
use networking::server::OutgoingReliableServerMessage;

use bevy::prelude::EventWriter;

/// Manage gridmap update events such as adding and removing cells.

pub(crate) fn gridmap_updates_manager(
    mut gridmap_main: ResMut<Gridmap>,
    sensers: Query<(Entity, &Senser, &ConnectedPlayer)>,
    mut server: EventWriter<OutgoingReliableServerMessage<GridmapServerMessage>>,
) {
    for (cell_id, cell_update) in gridmap_main.updates.iter_mut() {
        let cell_coords = to_doryen_coordinates(cell_id.x, cell_id.z);
        for (senser_entity, senser_component, connected_player_component) in sensers.iter() {
            if connected_player_component.connected
                && !cell_update.entities_received.contains(&senser_entity)
                && senser_component.fov.is_in_fov(cell_coords.0, cell_coords.1)
            {
                cell_update.entities_received.push(senser_entity);
                if cell_update.cell_data.item.id != 0 {
                    server.send(OutgoingReliableServerMessage {
                        handle: connected_player_component.handle,
                        message: GridmapServerMessage::AddCell(
                            cell_id.x,
                            cell_id.y,
                            cell_id.z,
                            cell_update.cell_data.item.id,
                            cell_update.cell_data.orientation.clone(),
                        ),
                    });
                } else {
                    server.send(OutgoingReliableServerMessage {
                        handle: connected_player_component.handle,
                        message: GridmapServerMessage::RemoveCell(cell_id.x, cell_id.y, cell_id.z),
                    });
                }
            }
        }
    }
}
