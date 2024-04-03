use bevy::prelude::{EventReader, Res};
use player::connections::SendServerConfiguration;

use crate::{grid::Gridmap, net::GridmapServerMessage};
use networking::server::OutgoingReliableServerMessage;

use bevy::prelude::EventWriter;

pub(crate) fn configure(
    mut config_events: EventReader<SendServerConfiguration>,
    mut server: EventWriter<OutgoingReliableServerMessage<GridmapServerMessage>>,
    gridmap_data: Res<Gridmap>,
) {
    for event in config_events.read() {
        server.send(OutgoingReliableServerMessage {
            handle: event.handle,
            message: GridmapServerMessage::ConfigOrderedCellsMain(
                gridmap_data.ordered_names.clone(),
            ),
        });
    }
}
