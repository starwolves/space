use bevy::{ecs::ResMut, prelude::info};
use bevy_networking_turbulence::NetworkResource;

use crate::space_core::structs::network_messages::{ReliableClientMessage, ReliableServerMessage};

pub fn handle_network_messages(mut net: ResMut<NetworkResource>) {
    for (handle, connection) in net.connections.iter_mut() {
        let channels = connection.channels().unwrap();
        while let Some(client_message) = channels.recv::<ReliableClientMessage>() {
            info!("ReliableClientMessage received on [{}]: {:?}",handle, client_message);

            match client_message {
                ReliableClientMessage::Awoo => {},
                ReliableClientMessage::UIInput(node_class,action,node_name,ui_type) => {

                    

                }
            }

        }

        while let Some(_server_message) = channels.recv::<ReliableServerMessage>() {
            // In case we ever get this from faulty or malicious clients, free it up.
        }

    }
}
