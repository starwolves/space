use bevy::{math::Vec3, prelude::{Entity, EventReader, EventWriter, Query, Res, warn}};
use bevy_rapier3d::{prelude::RigidBodyPosition};

use crate::space_core::{components::{pawn::Pawn, radio::Radio, sensable::Sensable, standard_character::StandardCharacter}, events::{general::{input_chat_message::InputChatMessage}, net::net_chat_message::NetChatMessage}, functions::new_chat_message::{Communicator, new_chat_message}, resources::handle_to_entity::HandleToEntity};

pub fn chat_message_input_event(
    mut chat_message_input_events: EventReader<InputChatMessage>,
    handle_to_entity : Res<HandleToEntity>,
    mut player_pawns : Query<(
        &Pawn,
        &RigidBodyPosition,
        &Sensable,
        &mut StandardCharacter,
    )>,
    radio_pawns : Query<(Entity, &Radio, &RigidBodyPosition)>,
    mut net_new_chat_message_event : EventWriter<NetChatMessage>
) {

    for chat_message_input_event in chat_message_input_events.iter() {

        
        let player_pawn_entity_option = handle_to_entity.map.get(&chat_message_input_event.handle);
        let player_pawn_entity;

        match player_pawn_entity_option {
            Some(entity) => {
                player_pawn_entity = entity;
            },
            None => {
                warn!("Couldn't find player pawn entity with handle_to_entity resource.");
                continue;
            },
        }

        let player_components_result = player_pawns.get_mut(*player_pawn_entity);

        
        
        match player_components_result {
            Ok(mut player_components) => {

                let player_position;
                
                let translation = player_components.1.position.translation;
                player_position = Vec3::new(
                    translation.x,
                    translation.y,
                    translation.z
                );
                    

                new_chat_message(
                    &mut net_new_chat_message_event,
                    &handle_to_entity,
                    &player_components.2.sensed_by,
                    &player_components.2.sensed_by_cached,
                    player_position,
                    player_components.0.name.clone(),
                    player_components.0.job,
                    chat_message_input_event.message.clone(),
                    Communicator::Standard,
                    false,
                    &radio_pawns,
                    Some(&player_pawn_entity),
                    Some(&mut player_components.3),
                );

            },
            Err(_) => {
                warn!("Couldn't find player player_components_result with query.get().");
                continue;
            },
        }


        
        



    }

}