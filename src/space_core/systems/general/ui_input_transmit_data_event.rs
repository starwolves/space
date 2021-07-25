use bevy::prelude::{Commands, EventReader, EventWriter, Query, Res, ResMut};

use crate::space_core::{components::{boarding::Boarding, connected_player::ConnectedPlayer, persistent_player_data::PersistentPlayerData}, events::general::{boarding_player::BoardingPlayer, ui_input_transmit_text::UIInputTransmitText}, functions::entity::new_chat_message::escape_bb, resources::{handle_to_entity::HandleToEntity, used_names::UsedNames}};

pub fn ui_input_transmit_data_event(
    mut event : EventReader<UIInputTransmitText>,
    mut boarding_player_event : EventWriter<BoardingPlayer>,
    handle_to_entity: Res<HandleToEntity>,
    mut used_names : ResMut<UsedNames>,
    mut query : Query<(&mut PersistentPlayerData, &ConnectedPlayer)>,
    mut commands : Commands
) {


    for new_event in event.iter() {

        let player_entity = handle_to_entity.map.get(&new_event.handle)
        .expect("ui_input_transmit_text_event.rs could not find entity belonging to player handle.");

        let player_components = query.get_mut(*player_entity)
        .expect("ui_input_transmit_text_event.rs could not find components belonging to player.");

        let mut persistent_player_data = player_components.0;
        let connected_player_component = player_components.1;

        if new_event.ui_type == "setupUI" {

            if new_event.node_path == 
            "HBoxContainer/Control/TabContainer/Character/VBoxContainer/vBoxNameInput/Control/inputName" {
                // In the future check if we have recieved all requested data sets and THEN remove Boarding component.
                
                persistent_player_data.character_name = escape_bb(new_event.input_text.clone(), true);

                if used_names.names.contains(&persistent_player_data.character_name) {
                    // Character name of player is already in-use.
                    continue;
                }
                if persistent_player_data.character_name.len() < 3 {
                    continue;
                }

                used_names.names.push(persistent_player_data.character_name.clone());

                commands.entity(*player_entity).remove::<Boarding>();

                boarding_player_event.send(BoardingPlayer{
                    entity: *player_entity,
                    player_handle: connected_player_component.handle,
                    player_character_name: persistent_player_data.character_name.clone(),
                });
    
            }
    
        }


    }


}
