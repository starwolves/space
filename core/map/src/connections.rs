use crate::map_input::MapData;
use bevy::prelude::{EventReader, Res};
use player::connection::SendServerConfiguration;

use bevy::prelude::ResMut;
use bevy_renet::renet::RenetServer;
#[cfg(feature = "server")]
pub(crate) fn configure(
    mut config_events: EventReader<SendServerConfiguration>,
    map_data: Res<MapData>,
    mut server: ResMut<RenetServer>,
) {
    use networking::plugin::RENET_RELIABLE_CHANNEL_ID;

    use crate::networking::MapServerMessage;

    for event in config_events.iter() {
        for add in map_data.to_net() {
            server.send_message(
                event.handle,
                RENET_RELIABLE_CHANNEL_ID,
                bincode::serialize(&MapServerMessage::MapDefaultAddition(add.0, add.1, add.2))
                    .unwrap(),
            );
        }
    }
}
