use std::collections::HashMap;

use bevy::prelude::{Added, Commands, EventReader, EventWriter, Query, Res, Resource};
use networking::server::HandleToEntity;
use player::connections::{PlayerAwaitingBoarding, PlayerServerMessage};

/// Godot NodePath.
pub const INPUT_NAME_PATH_FULL : &str = "setupUI::ColorRect/background/VBoxContainer/HBoxContainer/characterSettingsPopup/Control/TabContainer/Boarding Configuration/VBoxContainer/vBoxNameInput/Control/inputName";
/// Godot NodePath.
pub const INPUT_NAME_PATH : &str = "ColorRect/background/VBoxContainer/HBoxContainer/characterSettingsPopup/Control/TabContainer/Boarding Configuration/VBoxContainer/vBoxNameInput/Control/inputName";

use motd::motd::MOTD;
use player::name_generator::get_full_name;
use player::{connection::SetupPhase, names::UsedNames};

use networking::server::NetworkingChatServerMessage;
use networking::server::OutgoingReliableServerMessage;

use networking::server::ConnectedPlayer;
/// Initialize the setup UI by spawning in showcase entities etc.
#[cfg(feature = "server")]
pub(crate) fn initialize_setupui(
    used_names: Res<UsedNames>,
    query: Query<&ConnectedPlayer, Added<SetupPhase>>,
    mut server1: EventWriter<OutgoingReliableServerMessage<NetworkingChatServerMessage>>,
    mut server2: EventWriter<OutgoingReliableServerMessage<SetupUiServerMessage>>,
    motd: Res<MOTD>,
) {
    for connected_player_component in query.iter() {
        let suggested_name = get_full_name(true, true, &used_names);
        server2.send(OutgoingReliableServerMessage {
            handle: connected_player_component.handle,
            message: SetupUiServerMessage::SuggestedCharacterName(suggested_name.clone()),
        });
        server1.send(OutgoingReliableServerMessage {
            handle: connected_player_component.handle,
            message: NetworkingChatServerMessage::ChatMessage(motd.message.clone()),
        });
    }
}
use bevy::prelude::warn;
use console_commands::networking::ConsoleCommandsServerMessage;
use player::boarding::{BoardingPlayer, PersistentPlayerData};
use serde::{Deserialize, Serialize};
use text_api::core::CONSOLE_ERROR_COLOR;

use bevy::prelude::ResMut;
use text_api::core::escape_bb;

#[cfg(feature = "server")]
pub(crate) fn receive_input_character_name(
    mut server: EventReader<IncomingReliableClientMessage<SetupUiClientMessage>>,
    handle_to_entity: Res<HandleToEntity>,
    mut persistent_query: Query<&mut PersistentPlayerData>,
) {
    for message in server.iter() {
        match message.message.clone() {
            SetupUiClientMessage::InputCharacterName(name) => {
                match handle_to_entity.map.get(&message.handle) {
                    Some(entity) => match persistent_query.get_mut(*entity) {
                        Ok(mut component) => {
                            component.character_name = name;
                        }
                        Err(_) => {
                            warn!("coudlnt find entity in query receive_input_character_name");
                            continue;
                        }
                    },
                    None => {
                        warn!("Couldnt find entity of handle receive_input_character_name");
                        continue;
                    }
                }
            }
            _ => (),
        }
    }
}

/// Recieve boarding UI input.
#[cfg(feature = "server")]
pub(crate) fn ui_input_boarding(
    mut event: EventReader<IncomingReliableClientMessage<SetupUiClientMessage>>,
    mut boarding_player_event: EventWriter<BoardingPlayer>,
    handle_to_entity: Res<HandleToEntity>,
    used_names: ResMut<UsedNames>,
    mut query: Query<(&mut PersistentPlayerData, &ConnectedPlayer)>,
    mut server: EventWriter<OutgoingReliableServerMessage<ConsoleCommandsServerMessage>>,
) {
    for new_event in event.iter() {
        let player_entity = handle_to_entity.map.get(&new_event.handle).expect(
            "ui_input_transmit_text_event.rs could not find entity belonging to player handle.",
        );

        match new_event.message {
            SetupUiClientMessage::RequestBoarding => {
                let player_components;

                match query.get_mut(*player_entity) {
                    Ok(s) => {
                        player_components = s;
                    }
                    Err(_rr) => {
                        warn!("ui_input_transmit_text_event.rs could not find components belonging to player entity: {:?}", player_entity);
                        continue;
                    }
                }

                let mut persistent_player_data = player_components.0;
                let connected_player_component = player_components.1;

                persistent_player_data.character_name = escape_bb(
                    persistent_player_data.character_name.to_string(),
                    true,
                    true,
                );

                if persistent_player_data.character_name.len() > 26 {
                    persistent_player_data.character_name =
                        persistent_player_data.character_name[..26].to_string();
                }

                let mut name_in_use = false;

                for name in used_names.names.keys() {
                    if name.to_lowercase() == persistent_player_data.character_name.to_lowercase() {
                        // Character name of player is already in-use.
                        name_in_use = true;
                        break;
                    }
                }

                if name_in_use {
                    // Character name of player is already in-use.
                    server.send(OutgoingReliableServerMessage {
                        handle: new_event.handle,
                        message: ConsoleCommandsServerMessage::ConsoleWriteLine(
                            "[color=".to_string()
                                + CONSOLE_ERROR_COLOR
                                + "]Character name is already in-use.[/color]",
                        ),
                    });
                    continue;
                }

                if persistent_player_data.character_name.len() < 3 {
                    server.send(OutgoingReliableServerMessage {
                        handle: new_event.handle,
                        message: ConsoleCommandsServerMessage::ConsoleWriteLine(
                            "[color=".to_string()
                                + CONSOLE_ERROR_COLOR
                                + "]Character name is too short.[/color]",
                        ),
                    });
                    continue;
                }

                boarding_player_event.send(BoardingPlayer {
                    entity: *player_entity,
                    player_handle: connected_player_component.handle,
                    player_character_name: persistent_player_data.character_name.clone(),
                });
            }
            _ => (),
        }
    }
}

/// Sets radio channel list for clients in setup UI to only show global chat availability as a function.
#[cfg(feature = "server")]
pub fn get_talk_spaces_setupui() -> Vec<(String, String)> {
    use text_api::core::TALK_SPACE_GLOBAL_CHATPREFIX;

    vec![(
        "Global".to_string(),
        TALK_SPACE_GLOBAL_CHATPREFIX.to_string(),
    )]
}

use player::connection::SendServerConfiguration;

#[cfg(feature = "server")]
pub(crate) fn configure(
    mut config_events: EventReader<SendServerConfiguration>,
    mut server: EventWriter<OutgoingReliableServerMessage<PlayerServerMessage>>,
    mut server1: EventWriter<OutgoingReliableServerMessage<SetupUiServerMessage>>,
) {
    for event in config_events.iter() {
        let talk_spaces = get_talk_spaces_setupui();

        server.send(OutgoingReliableServerMessage {
            handle: event.handle,
            message: PlayerServerMessage::ConfigTalkSpaces(talk_spaces),
        });
        server1.send(OutgoingReliableServerMessage {
            handle: event.handle,
            message: SetupUiServerMessage::InitSetupUi,
        });
    }
}

/// Setup ui state resource.
#[cfg(feature = "server")]
#[derive(Resource, Default)]
pub struct SetupUiState {
    pub enabled: HashMap<u64, bool>,
}

/// Show setup_ui to newly connected clients.
#[cfg(feature = "server")]
pub(crate) fn new_clients_enable_setupui(
    mut player_awaiting_boarding: EventReader<PlayerAwaitingBoarding>,
    mut state: ResMut<SetupUiState>,
) {
    for awaiting in player_awaiting_boarding.iter() {
        if !state.enabled.contains_key(&awaiting.handle) {
            state.enabled.insert(awaiting.handle, true);
        }
    }
}
use networking::client::IncomingReliableServerMessage;
use networking::client::OutgoingReliableClientMessage;
use typename::TypeName;

/// Receive message from server to initialize setup ui.
#[cfg(feature = "client")]
pub(crate) fn client_setup_ui(
    mut incoming_setupui_messages: EventReader<IncomingReliableServerMessage<SetupUiServerMessage>>,
    mut outgoing_setupui_messages: EventWriter<OutgoingReliableClientMessage<SetupUiClientMessage>>,
) {
    for message in incoming_setupui_messages.iter() {
        let player_message = message.message.clone();
        match player_message {
            SetupUiServerMessage::SuggestedCharacterName(name) => {
                outgoing_setupui_messages.send(OutgoingReliableClientMessage {
                    message: SetupUiClientMessage::InputCharacterName(name),
                });
                outgoing_setupui_messages.send(OutgoingReliableClientMessage {
                    message: SetupUiClientMessage::RequestBoarding,
                });
            }
            SetupUiServerMessage::InitSetupUi => {
                outgoing_setupui_messages.send(OutgoingReliableClientMessage {
                    message: SetupUiClientMessage::SetupUiLoaded,
                });
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum SetupUiServerMessage {
    SuggestedCharacterName(String),
    InitSetupUi,
}
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum SetupUiClientMessage {
    InputCharacterName(String),
    SetupUiLoaded,
    RequestBoarding,
}
use networking::server::IncomingReliableClientMessage;
/// Manage when client has finished loading in a scene.
#[cfg(feature = "server")]
pub fn setupui_loaded(
    mut event: EventReader<IncomingReliableClientMessage<SetupUiClientMessage>>,
    handle_to_entity: Res<HandleToEntity>,
    mut commands: Commands,
) {
    for new_event in event.iter() {
        let player_entity = handle_to_entity.map.get(&new_event.handle)
        .expect("scene_ready_event.rs could not find components for player that just got done boarding.");
        commands.entity(*player_entity).insert(SetupPhase);
    }
}
