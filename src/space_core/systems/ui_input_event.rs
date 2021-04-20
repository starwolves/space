use bevy::prelude::{Commands, EventReader, Res};

use crate::space_core::{
    events::ui_input::UIInput,
    resources::handle_to_entity::HandleToEntity,
    structs::network_messages::{
        UIInputAction, 
        UIInputNodeClass
    },
    components::{
        boarding::Boarding
    }
};

pub fn ui_input_event(
    mut event : EventReader<UIInput>,
    handle_to_entity: Res<HandleToEntity>,
    mut commands : Commands
) {

    

    for new_event in event.iter() {

        let player_entity = handle_to_entity.map.get(&new_event.handle)
        .expect("ui_input_event.rs could not find components for player that just got done boarding.");


        if new_event.ui_type == "setupUI" {

            if new_event.node_name == "board" && 
            matches!(new_event.node_class, UIInputNodeClass::Button) && 
            matches!(new_event.action, UIInputAction::Pressed) {

                commands.entity(*player_entity).insert(Boarding);

            }

        }

    }

}
