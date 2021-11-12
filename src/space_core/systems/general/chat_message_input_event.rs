use bevy::{math::Vec3, prelude::{Entity, EventReader, EventWriter, Query, Res, warn}};
use bevy_rapier3d::{prelude::RigidBodyPosition};

use crate::space_core::{components::{connected_player::ConnectedPlayer, pawn::{Pawn, SpaceJobsEnum}, persistent_player_data::PersistentPlayerData, radio::Radio, sensable::Sensable, soft_player::SoftPlayer}, events::{general::{input_chat_message::InputChatMessage}, net::{net_chat_message::NetChatMessage, net_send_entity_updates::NetSendEntityUpdates}}, functions::entity::new_chat_message::{Communicator, MessagingPlayerState, new_chat_message}, resources::handle_to_entity::HandleToEntity};

pub fn chat_message_input_event(
    mut chat_message_input_events: EventReader<InputChatMessage>,
    handle_to_entity : Res<HandleToEntity>,
    player_pawns : Query<(
        &Pawn,
        &RigidBodyPosition,
        &Sensable,
    )>,
    radio_pawns : Query<(Entity, &Radio, &RigidBodyPosition, &PersistentPlayerData)>,
    soft_player_query : Query<&SoftPlayer>,
    mut net_new_chat_message_event : EventWriter<NetChatMessage>,
    mut net_send_entity_updates: EventWriter<NetSendEntityUpdates>,
    global_listeners : Query<(&ConnectedPlayer, &PersistentPlayerData)>,
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
                    &global_listeners,
                    Some(&player_pawn_entity),
                    Some(&mut net_send_entity_updates),
                    &MessagingPlayerState::Alive,
                );

            },
            Err(_) => {
                // Soft connected chat

                let persistent_player_data_component;

                //Safety check.
                match soft_player_query.get(*player_pawn_entity) {
                    Ok(_) => {},
                    Err(_rr) => {continue;},
                }

                match global_listeners.get(*player_pawn_entity) {
                    Ok((_connected, persistent_data)) => {
                        persistent_player_data_component = persistent_data;
                    },
                    Err(_rr) => {
                        warn!("Couldnt find components for SoftConnected player with assumed global message.");
                        continue;
                    },
                }

                new_chat_message(
                    &mut net_new_chat_message_event,
                    &handle_to_entity,
                    &vec![],
                    &vec![],
                    Vec3::ZERO,
                    persistent_player_data_component.user_name.clone(),
                    SpaceJobsEnum::Security,
                    chat_message_input_event.message.clone(),
                    Communicator::Standard,
                    false,
                    &radio_pawns,
                    &global_listeners,
                    Some(&player_pawn_entity),
                    Some(&mut net_send_entity_updates),
                    &MessagingPlayerState::SoftConnected,
                );
            },
        }

    }

}
