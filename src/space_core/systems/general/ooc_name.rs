use bevy::prelude::{EventReader, EventWriter, Query, ResMut, warn};

use crate::space_core::{components::persistent_player_data::PersistentPlayerData, events::{general::input_ooc_name::InputOocName, net::net_ooc_name::NetOocName}, functions::{console_commands::CONSOLE_ERROR_COLOR, entity::new_chat_message::escape_bb}, resources::{network_messages::ReliableServerMessage, used_names::UsedNames}};

pub fn ooc_name(

    mut input_ooc_name_events : EventReader<InputOocName>,
    mut persistent_player_data_query: Query<&mut PersistentPlayerData>,
    mut used_names : ResMut<UsedNames>,
    mut net_ooc_name_event : EventWriter<NetOocName>,

) {

    for event in input_ooc_name_events.iter() {

        match persistent_player_data_query.get_mut(event.entity) {
            Ok(mut persistent_player_data_component) => {

                let mut ooc_name = escape_bb((&event.input_name).to_string(), true, true);

                if ooc_name.len() > 16 {
                    ooc_name = ooc_name[..16].to_string();
                }

                if used_names.ooc_names.contains_key(&ooc_name) {
                    //Already exists.
                    warn!("User provided an OOC name that is already in-use.");

                    net_ooc_name_event.send(NetOocName{
                        handle: event.handle,
                        message: ReliableServerMessage::ConsoleWriteLine("[color=".to_string() + CONSOLE_ERROR_COLOR + "]The provided ooc_name in /content/init.json is already in-use, please change the name in the file and restart your game.[/color]"),
                    });

                    continue;
                }

                if ooc_name.len() < 3 {
                    net_ooc_name_event.send(NetOocName {
                        handle: event.handle,
                        message: ReliableServerMessage::ConsoleWriteLine("[color=".to_string() + CONSOLE_ERROR_COLOR + "]The provided ooc_name is too short. Special characters aren't registered.[/color]"),
                    });
                    continue;
                }

                persistent_player_data_component.ooc_name = ooc_name.to_string();

                used_names.ooc_names.insert(ooc_name, event.entity);

            },
            Err(_rr) => {
                warn!("Couldnt find persistent_player_data_component in query.");
            },
        }

    }

}
