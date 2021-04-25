use bevy::prelude::{Commands, EventReader, Query, Res, ResMut};

use crate::space_core::{components::{boarding::Boarding, persistent_player_data::PersistentPlayerData}, events::general::ui_input_transmit_text::UIInputTransmitText, resources::{handle_to_entity::HandleToEntity, used_names::UsedNames}};

pub fn ui_input_transmit_text_event(
    mut event : EventReader<UIInputTransmitText>,
    handle_to_entity: Res<HandleToEntity>,
    mut used_names : ResMut<UsedNames>,
    mut query : Query<&mut PersistentPlayerData>,
    mut commands : Commands
) {


    for new_event in event.iter() {

        let player_entity = handle_to_entity.map.get(&new_event.handle)
        .expect("ui_input_transmit_text_event.rs could not find entity belonging to player handle.");

        let mut persistent_player_data = query.get_mut(*player_entity)
        .expect("ui_input_transmit_text_event.rs could not find components belonging to player.");

        if new_event.ui_type == "setupUI" {

            if new_event.node_path == 
            "HBoxContainer/Control/TabContainer/Character/VBoxContainer/vBoxNameInput/Control/inputName" {
                // In the future check if we have recieved all requested data sets and THEN remove Boarding component.
                
                persistent_player_data.character_name = new_event.input_text.clone();

                if used_names.names.contains(&persistent_player_data.character_name) {
                    // Character name of player is already in-use.
                    continue;
                }

                used_names.names.push(persistent_player_data.character_name.clone());

                commands.entity(*player_entity).remove::<Boarding>();  
    
            }
    
        }


    }


}
