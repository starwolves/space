use std::collections::HashMap;

use bevy::prelude::{Added, EventWriter, Query, Res};

use crate::space_core::{
    components::{
        connected_player::ConnectedPlayer, 
        setup_phase::SetupPhase
    }, 
    events::net::net_on_setupui::NetOnSetupUI,
    functions::name_generator,
    resources::{server_id::ServerId, used_names::UsedNames},
    structs::network_messages::{EntityUpdateData, ReliableServerMessage}};

pub fn on_setupui (
    used_names : Res<UsedNames>,
    server_id : Res<ServerId>,
    query : Query<&ConnectedPlayer,Added<SetupPhase>>,
    mut net_on_setupui : EventWriter<NetOnSetupUI>
) {
    
    for connected_player_component in query.iter() {

        let suggested_name = name_generator::get_full_name(true, true, &used_names);

        let mut hash_map_data = HashMap::new();

        hash_map_data.insert("UIText".to_string(), EntityUpdateData::String(suggested_name));
        
        let mut hash_map_path = HashMap::new();

        hash_map_path.insert(
            "setupUI::HBoxContainer/Control/TabContainer/Character/VBoxContainer/vBoxNameInput/Control/inputName".to_string(),
            hash_map_data
        );


        net_on_setupui.send(NetOnSetupUI{
            handle: connected_player_component.handle,
            message: ReliableServerMessage::EntityUpdate(
                server_id.id.id(),
                hash_map_path
            )
        });

    }

}
