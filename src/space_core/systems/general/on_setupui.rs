use std::collections::HashMap;

use bevy::prelude::{Added, Commands, EventWriter, Query, Res, Transform};

use crate::space_core::{bundles::human_male_pawn::HumanMalePawnBundle, components::{connected_player::ConnectedPlayer, persistent_player_data::PersistentPlayerData, setup_phase::SetupPhase}, events::net::{net_on_setupui::NetOnSetupUI, net_showcase::NetShowcase}, functions::entity::name_generator, resources::{network_messages::{EntityUpdateData, ReliableServerMessage}, server_id::ServerId, used_names::UsedNames}};

pub fn on_setupui (
    used_names : Res<UsedNames>,
    server_id : Res<ServerId>,
    query : Query<(&ConnectedPlayer, &PersistentPlayerData),Added<SetupPhase>>,
    mut net_on_setupui : EventWriter<NetOnSetupUI>,
    mut net_showcase : EventWriter<NetShowcase>,
    mut commands : Commands,
) {
    
    for (connected_player_component, persistent_player_data_component) in query.iter() {

        let suggested_name = name_generator::get_full_name(true, true, &used_names);

        let mut hash_map_data = HashMap::new();

        hash_map_data.insert("label_text".to_string(), EntityUpdateData::String(suggested_name));
        
        let mut hash_map_path = HashMap::new();

        hash_map_path.insert(
            "setupUI::HBoxContainer/Control/TabContainer/Character/VBoxContainer/vBoxNameInput/Control/inputName".to_string(),
            hash_map_data
        );


        net_on_setupui.send(NetOnSetupUI{
            handle: connected_player_component.handle,
            message: ReliableServerMessage::EntityUpdate(
                server_id.id.to_bits(),
                hash_map_path,
                false,
            )
        });

        let passed_inventory_setup = vec![
            ("jumpsuit".to_string(), "jumpsuitSecurity".to_string()),
        ];

        HumanMalePawnBundle::spawn(
            Transform::identity(),
            &mut commands,
            persistent_player_data_component,
            connected_player_component,
            passed_inventory_setup,
            true,
            Some(&mut net_showcase),
            true,
        );

        

    }

}
