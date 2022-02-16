use bevy::prelude::{warn, Commands, EventReader, EventWriter, Query, Res, ResMut};

use crate::space::core::{
    networking::resources::ReliableServerMessage,
    pawn::{
        components::{Boarding, ConnectedPlayer, PersistentPlayerData},
        events::{BoardingPlayer, InputUIInputTransmitText, NetUIInputTransmitData},
        functions::{new_chat_message::escape_bb, CONSOLE_ERROR_COLOR},
        resources::{HandleToEntity, UsedNames},
        systems::on_setupui::INPUT_NAME_PATH,
    },
};

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
