use bevy::prelude::warn;
use crate::space_core::structs::network_messages::*;

pub fn on_new_connection() {
    match net.send_message(*handle, ReliableServerMessage::ConfigMessage(ConfigMessage::WorldEnvironment(*world_environment))) {
        Ok(msg) => match msg {
            Some(msg) => {
                warn!("handle_network_events.rs NetworkEvent::Connected: was unable to send WorldEnvironment: {:?}", msg);
            }
            None => {}
        },
        Err(err) => {
            warn!("handle_network_events.rs NetworkEvent::Connected: was unable to send WorldEnvironment (1): {:?}", err);
        }
    };
}
