use crate::map_input::MapData;
use bevy::prelude::{EventReader, EventWriter, Res};
use networking::server::PendingMessage;
use networking::server::PendingNetworkMessage;
use networking::server::ReliableServerMessage;
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
    map_data: Res<MapData>,
) {
    for event in config_events.iter() {
        for add in map_data.to_net() {
            net_on_new_player_connection.send(NetConfig {
                handle: event.handle,
                message: ReliableServerMessage::MapDefaultAddition(add.0, add.1, add.2),
            });
        }
    }
}
