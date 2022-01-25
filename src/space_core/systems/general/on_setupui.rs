use std::collections::HashMap;

use bevy::{prelude::{Added, Commands, EventWriter, Query, Res, ResMut, Transform}};

use crate::space_core::{bundles::human_male_pawn::HumanMalePawnBundle, components::{connected_player::ConnectedPlayer, persistent_player_data::PersistentPlayerData, setup_phase::SetupPhase}, events::net::{net_on_setupui::NetOnSetupUI, net_showcase::NetShowcase}, functions::entity::name_generator, resources::{entity_data_resource::{EntityDataResource, SpawnPawnData}, motd::MOTD, network_messages::{EntityUpdateData, EntityWorldType, ReliableServerMessage}, server_id::ServerId, used_names::UsedNames}};

pub const INPUT_NAME_PATH_FULL : &str = "setupUI::ColorRect/background/VBoxContainer/HBoxContainer/characterSettingsPopup/Control/TabContainer/Boarding Configuration/VBoxContainer/vBoxNameInput/Control/inputName";
pub const INPUT_NAME_PATH : &str = "ColorRect/background/VBoxContainer/HBoxContainer/characterSettingsPopup/Control/TabContainer/Boarding Configuration/VBoxContainer/vBoxNameInput/Control/inputName";
pub const ENTITY_SPAWN_PARENT : &str = "ColorRect/background/VBoxContainer/HBoxContainer/3dviewportPopup/Control/TabContainer/3D Viewport/Control/ViewportContainer/Viewport/Spatial";





pub fn on_setupui(
    used_names : Res<UsedNames>,
    server_id : Res<ServerId>,

    query : Query<(&ConnectedPlayer, &PersistentPlayerData),Added<SetupPhase>>,
    mut net_showcase : EventWriter<NetShowcase>,

    entity_data : ResMut<EntityDataResource>,
    
    mut net_on_setupui : EventWriter<NetOnSetupUI>,
    mut commands : Commands,
    motd : Res<MOTD>,
) {
    
    for (connected_player_component, persistent_player_data_component) in query.iter() {

        let suggested_name = name_generator::get_full_name(true, true, &used_names);

        let mut hash_map_data = HashMap::new();

        hash_map_data.insert("label_text".to_string(), EntityUpdateData::String(suggested_name));
        
        let mut hash_map_path = HashMap::new();

        hash_map_path.insert(
            INPUT_NAME_PATH_FULL.to_string(),
            hash_map_data
        );

        net_on_setupui.send(NetOnSetupUI{
            handle: connected_player_component.handle,
            message: ReliableServerMessage::EntityUpdate(
                server_id.id.to_bits(),
                hash_map_path,
                false,
                EntityWorldType::Main,
            )
        });

        net_on_setupui.send(NetOnSetupUI{
            handle: connected_player_component.handle,
            message: ReliableServerMessage::ChatMessage(motd.message.clone()),
        });

        let passed_inventory_setup = vec![
            ("jumpsuit".to_string(), "jumpsuitSecurity".to_string()),
            ("holster".to_string(), "pistolL1".to_string()),
        ];

        HumanMalePawnBundle::spawn(
            Transform::identity(),
            &mut commands,
            true,
            Some(SpawnPawnData {
                data: (
                    persistent_player_data_component,   
                    Some(connected_player_component),
                    passed_inventory_setup,
                    true,
                    false,
                    None,
                    Some(&mut net_showcase),
                    None,
                    &entity_data,
                )
            }),
            None,
        );

        
    }

}
