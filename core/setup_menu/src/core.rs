use std::collections::HashMap;

use bevy::prelude::{Added, Commands, EventReader, EventWriter, Query, Res, Resource};
use networking::server::HandleToEntity;
use player::connections::PlayerAwaitingBoarding;

/// Godot NodePath.
pub const INPUT_NAME_PATH_FULL : &str = "setupUI::ColorRect/background/VBoxContainer/HBoxContainer/characterSettingsPopup/Control/TabContainer/Boarding Configuration/VBoxContainer/vBoxNameInput/Control/inputName";
/// Godot NodePath.
pub const INPUT_NAME_PATH : &str = "ColorRect/background/VBoxContainer/HBoxContainer/characterSettingsPopup/Control/TabContainer/Boarding Configuration/VBoxContainer/vBoxNameInput/Control/inputName";

use motd::motd::MOTD;
use player::name_generator::get_full_name;
use player::{connections::SetupPhase, names::UsedNames};

use networking::server::NetworkingChatServerMessage;
use networking::server::OutgoingReliableServerMessage;

use networking::server::ConnectedPlayer;
/// Initialize the setup UI by spawning in showcase entities etc.

pub(crate) fn initialize_setupui(
    used_names: Res<UsedNames>,
    query: Query<&ConnectedPlayer, Added<SetupPhase>>,
    mut server1: EventWriter<OutgoingReliableServerMessage<NetworkingChatServerMessage>>,
    mut server2: EventWriter<OutgoingReliableServerMessage<SetupUiServerMessage>>,
    motd: Res<MOTD>,
    mut datas: ResMut<SetupUiUserDataSets>,
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
        datas.list.insert(
            connected_player_component.handle,
            SetupUiUserData {
                character_name: suggested_name,
            },
        );
    }
}
use bevy::prelude::warn;
use player::boarding::BoardingPlayer;

use bevy::prelude::ResMut;
#[derive(Default, Resource)]

/// Each stored [SetupUiState] for the connected handles.
pub struct SetupUiUserDataSets {
    pub list: HashMap<u64, SetupUiUserData>,
}

pub struct SetupUiUserData {
    pub character_name: String,
}

pub(crate) fn receive_input_character_name(
    mut server: EventReader<IncomingReliableClientMessage<SetupUiClientMessage>>,
    mut datas: ResMut<SetupUiUserDataSets>,
) {
    for message in server.iter() {
        match message.message.clone() {
            SetupUiClientMessage::InputCharacterName(name) => {
                match datas.list.get_mut(&message.handle) {
                    Some(setupui_data) => {
                        setupui_data.character_name = name;
                    }
                    None => {
                        warn!("Could not find SetupUiData for handle {}", message.handle);
                    }
                }
            }
            _ => (),
        }
    }
}

/// Recieve boarding UI input.

pub(crate) fn ui_input_boarding(
    mut event: EventReader<IncomingReliableClientMessage<SetupUiClientMessage>>,
    mut boarding_player_event: EventWriter<BoardingPlayer>,
    handle_to_entity: Res<HandleToEntity>,
    mut query: Query<&ConnectedPlayer>,
    setupui_datas: Res<SetupUiUserDataSets>,
) {
    for new_event in event.iter() {
        let player_entity = handle_to_entity
            .map
            .get(&new_event.handle)
            .expect("ui_input_boarding could not find entity belonging to player handle.");

        match new_event.message {
            SetupUiClientMessage::RequestBoarding => {
                let connected_player_component;
                match query.get_mut(*player_entity) {
                    Ok(s) => {
                        connected_player_component = s;
                    }
                    Err(_rr) => {
                        warn!("ui_input_boarding could not find components belonging to player entity: {:?}", player_entity);
                        continue;
                    }
                }

                match setupui_datas.list.get(&connected_player_component.handle) {
                    Some(setupui_data) => {
                        boarding_player_event.send(BoardingPlayer {
                            entity: *player_entity,
                            player_handle: connected_player_component.handle,
                            player_character_name: setupui_data.character_name.clone(),
                        });
                    }
                    None => {
                        warn!("ui_input_boarding could not find setupui_datas belonging to player: {:?}", connected_player_component.handle);
                    }
                }
            }
            _ => (),
        }
    }
}

use player::connections::SendServerConfiguration;

pub(crate) fn configure(
    mut config_events: EventReader<SendServerConfiguration>,
    mut server1: EventWriter<OutgoingReliableServerMessage<SetupUiServerMessage>>,
) {
    for event in config_events.iter() {
        server1.send(OutgoingReliableServerMessage {
            handle: event.handle,
            message: SetupUiServerMessage::InitSetupUi,
        });
    }
}

/// Setup ui state resource.

#[derive(Resource, Default)]
pub struct SetupUiState {
    pub enabled: HashMap<u64, bool>,
}

/// Show setup_ui to newly connected clients.

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
use crate::net::SetupUiServerMessage;
use networking::client::IncomingReliableServerMessage;
use networking::client::OutgoingReliableClientMessage;

/// Receive message from server to initialize setup ui.

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
use crate::net::SetupUiClientMessage;

use networking::server::IncomingReliableClientMessage;
/// Manage when client has finished loading in a scene.

pub fn setupui_loaded(
    mut event: EventReader<IncomingReliableClientMessage<SetupUiClientMessage>>,
    handle_to_entity: Res<HandleToEntity>,
    mut commands: Commands,
) {
    for new_event in event.iter() {
        match new_event.message {
            SetupUiClientMessage::SetupUiLoaded => {
                let player_entity = handle_to_entity.map.get(&new_event.handle)
        .expect("scene_ready_event.rs could not find components for player that just got done boarding.");
                commands.entity(*player_entity).insert(SetupPhase);
            }
            _ => {}
        }
    }
}
