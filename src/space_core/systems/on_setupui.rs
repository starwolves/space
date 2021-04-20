use bevy::prelude::{Added, Query, Res,ResMut, warn};
use bevy_networking_turbulence::NetworkResource;

use crate::space_core::{
    components::{
        connected_player::ConnectedPlayer, 
        setup_phase::SetupPhase
    },
    functions::name_generator,
    resources::{server_id::ServerId, used_names::UsedNames},
    structs::network_messages::{EntityUpdateData, ReliableServerMessage}
};

pub fn on_setupui (
    mut net: ResMut<NetworkResource>,
    used_names : Res<UsedNames>,
    server_id : Res<ServerId>,
    query : Query<&ConnectedPlayer,Added<SetupPhase>>
) {
    
    for connected_player_component in query.iter() {

        let suggested_name = name_generator::get_full_name(true, true, &used_names);

        match net.send_message(connected_player_component.handle, ReliableServerMessage::EntityUpdate(
            server_id.id.id(),
            "setupUI::HBoxContainer/Control/TabContainer/Character/VBoxContainer/vBoxNameInput/Control/inputName".to_string(),
            EntityUpdateData::UIText(suggested_name.to_string())
        )) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("handle_network_messages.rs was unable to send suggested name: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("handle_network_messages.rs was unable to send suggested name (1): {:?}", err);
            }
        };

    }

}
