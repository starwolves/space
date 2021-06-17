use bevy::{math::Vec3, prelude::{Entity, EventReader, EventWriter, Query, Res, warn}};
use bevy_rapier3d::{physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet};

use crate::space_core::{components::{pawn::Pawn, radio::Radio, sensable::Sensable}, events::{general::input_chat_message::InputChatMessage, net::net_chat_message::NetChatMessage}, functions::new_chat_message::{Communicator, new_chat_message}, resources::handle_to_entity::HandleToEntity};

pub fn chat_message_input_event(
    mut chat_message_input_events: EventReader<InputChatMessage>,
    handle_to_entity : Res<HandleToEntity>,
    player_pawns : Query<(
        &Pawn,
        &RigidBodyHandleComponent,
        &Sensable
    )>,
    radio_pawns : Query<(Entity, &Radio, &RigidBodyHandleComponent)>,
    rigid_bodies: Res<RigidBodySet>,
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

        let player_components_result = player_pawns.get(*player_pawn_entity);

        
        
        match player_components_result {
            Ok(player_components) => {

                let player_rigid_body_handle_component = player_components.1;
                let player_position;

                let player_position_option = rigid_bodies.get(player_rigid_body_handle_component.handle());

                match player_position_option {
                    Some(rigid_body) => {
                        let translation = rigid_body.position().translation;
                        player_position = Vec3::new(
                            translation.x,
                            translation.y,
                            translation.z
                        );
                    },
                    None => {
                        warn!("Couldn't find player pawn rigid_body via rigid_bodies.get()");
                        continue;
                    },
                }

                new_chat_message(
                    &mut net_new_chat_message_event,
                    &handle_to_entity,
                    &player_components.2.sensed_by,
                    &player_components.2.sensed_by,
                    player_position,
                    player_components.0.name.clone(),
                    player_components.0.job,
                    chat_message_input_event.message.clone(),
                    Communicator::Standard,
                    false,
                    &radio_pawns,
                    &player_pawn_entity,
                    &rigid_bodies
                );

            },
            Err(_) => {
                warn!("Couldn't find player player_components_result with query.get().");
                continue;
            },
        }


        
        



    }

}