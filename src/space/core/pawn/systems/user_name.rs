use bevy_internal::prelude::{warn, EventReader, EventWriter, Query, ResMut};

use crate::space::core::{
    networking::resources::ReliableServerMessage,
    pawn::{
        components::PersistentPlayerData,
        events::{InputUserName, NetUserName},
        functions::{new_chat_message::escape_bb, CONSOLE_ERROR_COLOR},
        resources::UsedNames,
    },
};

pub fn user_name(
    mut input_user_name_events: EventReader<InputUserName>,
    mut persistent_player_data_query: Query<&mut PersistentPlayerData>,
    mut used_names: ResMut<UsedNames>,
    mut net_user_name_event: EventWriter<NetUserName>,
) {
    for event in input_user_name_events.iter() {
        match persistent_player_data_query.get_mut(event.entity) {
            Ok(mut persistent_player_data_component) => {
                if persistent_player_data_component.user_name_is_set {
                    continue;
                }

                let mut user_name = escape_bb((&event.input_name).to_string(), true, true);

                if user_name.len() > 16 {
                    user_name = user_name[..16].to_string();
                }

                if used_names.user_names.contains_key(&user_name) {
                    //Already exists.

                    net_user_name_event.send(NetUserName{
                        handle: event.handle,
                        message: ReliableServerMessage::ConsoleWriteLine("[color=".to_string() + CONSOLE_ERROR_COLOR + "]The provided user_name is already in-use, please change the name in the file and restart your game.[/color]"),
                    });

                    continue;
                }

                if user_name.len() < 3 {
                    net_user_name_event.send(NetUserName {
                        handle: event.handle,
                        message: ReliableServerMessage::ConsoleWriteLine("[color=".to_string() + CONSOLE_ERROR_COLOR + "]The provided user_name is too short. Special characters and whitespaces are not registered.[/color]"),
                    });
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
