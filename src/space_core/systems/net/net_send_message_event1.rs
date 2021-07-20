use bevy::prelude::{EventReader, ResMut, warn};
use bevy_networking_turbulence::NetworkResource;

use crate::space_core::events::net::net_showcase::NetShowcase;



pub fn net_send_messages_event1(
    mut net: ResMut<NetworkResource>,
    mut net_showcase : EventReader<NetShowcase>,
) {

    


    for new_event in net_showcase.iter() {

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unable to send net_showcase message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_showcase message (1): {:?}", err);
            }
        };

    }

    
    
    
    
}
