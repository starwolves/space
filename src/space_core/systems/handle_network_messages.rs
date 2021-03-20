use bevy::{ecs::ResMut, prelude::info};
use bevy_networking_turbulence::NetworkResource;

use crate::space_core::structs::network_messages::ClientMessage;

pub fn handle_network_messages(mut net: ResMut<NetworkResource>) {
    for (handle, connection) in net.connections.iter_mut() {
        let channels = connection.channels().unwrap();
        while let Some(client_message) = channels.recv::<ClientMessage>() {
            info!("ClientMessage received on [{}]: {:?}",handle, client_message);
        }
    }
}
