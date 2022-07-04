use crate::{
    core::{
        chat::message::{escape_bb, get_talk_spaces},
        connected_player::connection::{OnBoard, SetupPhase, SoftPlayer, Spawning},
        console_commands::commands::CONSOLE_ERROR_COLOR,
        gridmap::gridmap::SpawnPoints,
        networking::networking::{ReliableServerMessage, ServerConfigMessage},
        pawn::{pawn::PersistentPlayerData, user_name::UsedNames},
    },
    entities::asana::tick_asana_boarding_announcements::AsanaBoardingAnnouncements,
};

use super::{
    connection::{Boarding, ConnectedPlayer},
    net::{NetDoneBoarding, NetOnBoarding},
    plugin::HandleToEntity,
    ui::INPUT_NAME_PATH,
};

pub struct BoardingPlayer {
    pub player_handle: u64,
    pub player_character_name: String,
    pub entity: Entity,
}

pub fn done_boarding(
    mut spawn_points: ResMut<SpawnPoints>,
    mut net_done_boarding: EventWriter<NetDoneBoarding>,
    mut boarding_player_event: EventReader<BoardingPlayer>,
    mut commands: Commands,

    mut asana_boarding_announcements: ResMut<AsanaBoardingAnnouncements>,
) {
    for boarding_player in boarding_player_event.iter() {
        let player_character_name = boarding_player.player_character_name.clone();
        let player_handle = boarding_player.player_handle;
        let entity_id = boarding_player.entity;

        info!(
            "{} [{}] has boarded the spaceship.",
            player_character_name, player_handle
        );

        let assigned_spawn_transform = spawn_points.list[spawn_points.i].transform;

        commands
            .entity(entity_id)
            .insert_bundle((
                OnBoard,
                Spawning {
                    transform: assigned_spawn_transform,
                },
            ))
            .remove_bundle::<(SetupPhase, SoftPlayer)>();

        spawn_points.i += 1;

        if spawn_points.i >= spawn_points.list.len() {
            spawn_points.i = 0;
        }

        // Queue net_code message for client so he goes back to the main scene and ditches setupUI.
        net_done_boarding.send(NetDoneBoarding {
            handle: player_handle,
            message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::ChangeScene(
                true,
                "main".to_string(),
            )),
        });

        let talk_spaces = get_talk_spaces();

        net_done_boarding.send(NetDoneBoarding {
            handle: player_handle,
            message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::TalkSpaces(
                talk_spaces,
            )),
        });

        asana_boarding_announcements.announcements.push((
            ";Security Officer ".to_owned() + &player_character_name + " is now on board.",
            Timer::from_seconds(2., false),
        ));
    }
}

use bevy::{
    core::Timer,
    prelude::{info, warn, Added, Commands, Entity, EventReader, EventWriter, Query, Res, ResMut},
};

pub struct InputUIInputTransmitText {
    pub handle: u64,
    pub ui_type: String,
    pub node_path: String,
    pub input_text: String,
}

use super::net::NetUIInputTransmitData;

pub fn ui_input_transmit_data_event(
    mut event: EventReader<InputUIInputTransmitText>,
    mut boarding_player_event: EventWriter<BoardingPlayer>,
    handle_to_entity: Res<HandleToEntity>,
    used_names: ResMut<UsedNames>,
    mut query: Query<(&mut PersistentPlayerData, &Boarding, &ConnectedPlayer)>,
    mut commands: Commands,
    mut net_ui_input_transmit_data_event: EventWriter<NetUIInputTransmitData>,
) {
    for new_event in event.iter() {
        let player_entity = handle_to_entity.map.get(&new_event.handle).expect(
            "ui_input_transmit_text_event.rs could not find entity belonging to player handle.",
        );

        let player_components;

        match query.get_mut(*player_entity) {
            Ok(s) => {
                player_components = s;
            }
            Err(_rr) => {
                warn!("ui_input_transmit_text_event.rs could not find components belonging to player.");
                continue;
            }
        }

        let mut persistent_player_data = player_components.0;
        let connected_player_component = player_components.2;

        if new_event.ui_type == "setupUI" {
            if new_event.node_path == INPUT_NAME_PATH {
                // In the future check if we have recieved all requested data sets and THEN remove Boarding component.

                persistent_player_data.character_name =
                    escape_bb(new_event.input_text.to_string(), true, true);

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
                    net_ui_input_transmit_data_event.send(NetUIInputTransmitData {
                        handle: new_event.handle,
                        message: ReliableServerMessage::ConsoleWriteLine(
                            "[color=".to_string()
                                + CONSOLE_ERROR_COLOR
                                + "]Character name is already in-use.[/color]",
                        ),
                    });
                    continue;
                }

                if persistent_player_data.character_name.len() < 3 {
                    net_ui_input_transmit_data_event.send(NetUIInputTransmitData {
                        handle: new_event.handle,
                        message: ReliableServerMessage::ConsoleWriteLine(
                            "[color=".to_string()
                                + CONSOLE_ERROR_COLOR
                                + "]Character name is too short.[/color]",
                        ),
                    });
                    continue;
                }

                commands.entity(*player_entity).remove::<Boarding>();

                boarding_player_event.send(BoardingPlayer {
                    entity: *player_entity,
                    player_handle: connected_player_component.handle,
                    player_character_name: persistent_player_data.character_name.clone(),
                });
            }
        }
    }
}

pub fn on_boarding(
    query: Query<&ConnectedPlayer, Added<Boarding>>,
    mut net_on_boarding: EventWriter<NetOnBoarding>,
) {
    for connected_player_component in query.iter() {
        net_on_boarding.send(NetOnBoarding {
            handle: connected_player_component.handle,
            message: ReliableServerMessage::UIRequestInput(
                "setupUI".to_string(),
                INPUT_NAME_PATH.to_string(),
            ),
        });
    }
}
