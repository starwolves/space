pub fn user_name(
    mut input_user_name_events: EventReader<InputUserName>,
    mut persistent_player_data_query: Query<&mut PersistentPlayerData>,
    mut used_names: ResMut<UsedNames>,
    mut net_user_name_event: EventWriter<NetPawn>,
    handle_to_entity: Res<HandleToEntity>,
) {
    for event in input_user_name_events.iter() {
        match persistent_player_data_query.get_mut(event.entity) {
            Ok(mut persistent_player_data_component) => {
                if persistent_player_data_component.user_name_is_set {
                    continue;
                }

                let handle_option;

                match handle_to_entity.inv_map.get(&event.entity) {
                    Some(x) => {
                        handle_option = Some(x);
                    }
                    None => {
                        handle_option = None;
                    }
                }

                let mut user_name = escape_bb((&event.input_name).to_string(), true, true);

                if user_name.len() > 16 {
                    user_name = user_name[..16].to_string();
                }

                if used_names.user_names.contains_key(&user_name) {
                    //Already exists.

                    match handle_option {
                        Some(handle) => {
                            net_user_name_event.send(NetPawn{
                                handle: *handle,
                                message: ReliableServerMessage::ConsoleWriteLine("[color=".to_string() + CONSOLE_ERROR_COLOR + "]The provided user_name is already in-use, please change the name in the file and restart your game.[/color]"),
                            });
                        }
                        None => {}
                    }

                    continue;
                }

                if user_name.len() < 3 {
                    match handle_option {
                        Some(handle) => {
                            net_user_name_event.send(NetPawn {
                                handle: *handle,
                                message: ReliableServerMessage::ConsoleWriteLine("[color=".to_string() + CONSOLE_ERROR_COLOR + "]The provided user_name is too short. Special characters and whitespaces are not registered.[/color]"),
                            });
                        }
                        None => {}
                    }
                    continue;
                }

                persistent_player_data_component.user_name = user_name.to_string();

                used_names.user_names.insert(user_name, event.entity);

                persistent_player_data_component.user_name_is_set = true;
            }
            Err(_rr) => {
                warn!("Couldnt find persistent_player_data_component in query.");
            }
        }
    }
}

pub struct NetPawn {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
impl PendingMessage for NetPawn {
    fn get_message(&self) -> PendingNetworkMessage {
        PendingNetworkMessage {
            handle: self.handle,
            message: self.message.clone(),
        }
    }
}

use api::{
    chat::escape_bb,
    console_commands::CONSOLE_ERROR_COLOR,
    data::HandleToEntity,
    network::{PendingMessage, PendingNetworkMessage, ReliableServerMessage},
};
use bevy::prelude::{warn, EventReader, EventWriter, Query, Res, ResMut};
use networking::messages::InputUserName;

use crate::pawn::{PersistentPlayerData, UsedNames};
