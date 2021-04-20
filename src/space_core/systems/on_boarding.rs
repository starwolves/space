use bevy::prelude::{Added, Query, ResMut, warn};
use bevy_networking_turbulence::NetworkResource;

use crate::space_core::{
    components::{
        boarding::Boarding,
        connected_player::ConnectedPlayer
    },
    structs::network_messages::{ReliableServerMessage}
};

pub fn on_boarding(
    mut net: ResMut<NetworkResource>,
    query : Query<&ConnectedPlayer,Added<Boarding>>
) {

    for connected_player_component in query.iter() {

        match net.send_message(connected_player_component.handle, ReliableServerMessage::UIRequestInput(
            "setupUI".to_string(),
            "HBoxContainer/Control/TabContainer/Character/VBoxContainer/vBoxNameInput/Control/inputName".to_string()
        )) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("handle_network_messages.rs was unable to UIRequestInput: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("handle_network_messages.rs was unable to UIRequestInput (1): {:?}", err);
            }
        };

    }

}
