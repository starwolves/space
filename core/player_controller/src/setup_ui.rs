use std::collections::HashMap;

use bevy::prelude::{Added, Commands, EventReader, EventWriter, Query, Res};
use entity::{
    meta::SoftPlayer,
    spawn::{SpawnData, SpawnEvent},
};
use humanoid::{humanoid::HUMAN_MALE_ENTITY_NAME, user_name::UsedNames};
use jumpsuit_security::jumpsuit::JUMPSUIT_SECURITY_ENTITY_NAME;
use motd::motd::MOTD;
use networking::messages::{
    EntityUpdateData, EntityWorldType, InputUIInput, ReliableServerMessage, UIInputAction,
    UIInputNodeClass,
};
use pawn::pawn::{PawnDesignation, PersistentPlayerData};
use pistol_l1::pistol_l1::PISTOL_L1_ENTITY_NAME;
use server::core::{ConnectedPlayer, HandleToEntity, ServerId};
use showcase::core::ShowcaseData;

use crate::{connection::SpawnPawnData, humanoid::HumanMaleSummoner};

use super::{
    connection::{Boarding, SetupPhase},
    name_generator::get_full_name,
    net::NetOnSetupUI,
};

/// Process player requesting board.
pub(crate) fn register_ui_input_boarding(
    mut event: EventReader<InputUIInput>,
    handle_to_entity: Res<HandleToEntity>,
    criteria_query: Query<&SoftPlayer>,
    mut commands: Commands,
) {
    for new_event in event.iter() {
        let player_entity = handle_to_entity.map.get(&new_event.handle).expect(
            "ui_input_event.rs could not find components for player that just got done boarding.",
        );

        // Safety check.
        match criteria_query.get(*player_entity) {
            Ok(_) => {}
            Err(_rr) => {
                continue;
            }
        }

        if new_event.ui_type == "setupUI" {
            if new_event.node_name == "board"
                && matches!(new_event.node_class, UIInputNodeClass::Button)
                && matches!(new_event.action, UIInputAction::Pressed)
            {
                commands.entity(*player_entity).insert(Boarding);
            }
        }
    }
}

/// Godot NodePath.
pub const INPUT_NAME_PATH_FULL : &str = "setupUI::ColorRect/background/VBoxContainer/HBoxContainer/characterSettingsPopup/Control/TabContainer/Boarding Configuration/VBoxContainer/vBoxNameInput/Control/inputName";
/// Godot NodePath.
pub const INPUT_NAME_PATH : &str = "ColorRect/background/VBoxContainer/HBoxContainer/characterSettingsPopup/Control/TabContainer/Boarding Configuration/VBoxContainer/vBoxNameInput/Control/inputName";

/// Initialize the setup UI by spawning in UI entities etc.
pub(crate) fn initialize_setupui(
    used_names: Res<UsedNames>,
    server_id: Res<ServerId>,
    query: Query<(&ConnectedPlayer, &PersistentPlayerData), Added<SetupPhase>>,
    mut net_on_setupui: EventWriter<NetOnSetupUI>,
    mut summon_human_male: EventWriter<SpawnEvent<HumanMaleSummoner>>,
    mut commands: Commands,
    motd: Res<MOTD>,
) {
    for (connected_player_component, persistent_player_data_component) in query.iter() {
        let suggested_name = get_full_name(true, true, &used_names);

        let mut hash_map_data = HashMap::new();

        hash_map_data.insert(
            "label_text".to_string(),
            EntityUpdateData::String(suggested_name),
        );

        let mut hash_map_path = HashMap::new();

        hash_map_path.insert(INPUT_NAME_PATH_FULL.to_string(), hash_map_data);

        net_on_setupui.send(NetOnSetupUI {
            handle: connected_player_component.handle,
            message: ReliableServerMessage::EntityUpdate(
                server_id.id.to_bits(),
                hash_map_path,
                false,
                EntityWorldType::Main,
            ),
        });

        net_on_setupui.send(NetOnSetupUI {
            handle: connected_player_component.handle,
            message: ReliableServerMessage::ChatMessage(motd.message.clone()),
        });

        let passed_inventory_setup = vec![
            (
                "jumpsuit".to_string(),
                JUMPSUIT_SECURITY_ENTITY_NAME.to_string(),
            ),
            ("holster".to_string(), PISTOL_L1_ENTITY_NAME.to_string()),
        ];

        let human_male_entity = commands.spawn().id();

        summon_human_male.send(SpawnEvent {
            spawn_data: SpawnData {
                entity: human_male_entity,
                showcase_data_option: Some(ShowcaseData {
                    handle: connected_player_component.handle,
                }),
                entity_name: HUMAN_MALE_ENTITY_NAME.to_string(),
                ..Default::default()
            },
            summoner: HumanMaleSummoner {
                character_name: persistent_player_data_component.character_name.clone(),
                user_name: persistent_player_data_component.user_name.clone(),
                spawn_pawn_data: SpawnPawnData {
                    persistent_player_data: persistent_player_data_component.clone(),
                    connected_player_option: Some(connected_player_component.clone()),
                    inventory_setup: passed_inventory_setup,
                    designation: PawnDesignation::Showcase,
                },
            },
        });
    }
}
