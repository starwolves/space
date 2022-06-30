use bevy_ecs::{
    entity::Entity,
    event::{EventReader, EventWriter},
    system::{Query, Res, ResMut},
};
use bevy_log::warn;
use bevy_math::Vec3;
use bevy_renet::renet::RenetServer;
use bevy_transform::prelude::Transform;

use crate::core::{
    connected_player::{
        components::{ConnectedPlayer, SoftPlayer},
        resources::HandleToEntity,
    },
    entity::events::NetSendEntityUpdates,
    networking::{send_net, NetEvent, RENET_RELIABLE_CHANNEL_ID},
    pawn::components::{Pawn, PersistentPlayerData, ShipJobsEnum},
    sensable::components::Sensable,
};

use super::{
    components::Radio,
    events::{InputChatMessage, NetChatMessage},
    functions::{new_chat_message, Communicator, MessagingPlayerState},
};

pub fn chat_message_input_event(
    mut chat_message_input_events: EventReader<InputChatMessage>,
    handle_to_entity: Res<HandleToEntity>,
    player_pawns: Query<(&Pawn, &Transform, &Sensable)>,
    radio_pawns: Query<(Entity, &Radio, &Transform, &PersistentPlayerData)>,
    soft_player_query: Query<&SoftPlayer>,
    mut net_new_chat_message_event: EventWriter<NetChatMessage>,
    mut net_send_entity_updates: EventWriter<NetSendEntityUpdates>,
    global_listeners: Query<(&ConnectedPlayer, &PersistentPlayerData)>,
) {
    for chat_message_input_event in chat_message_input_events.iter() {
        let player_pawn_entity;
        player_pawn_entity = chat_message_input_event.entity;

        let player_components_result = player_pawns.get(player_pawn_entity);

        match player_components_result {
            Ok(player_components) => {
                let player_position;

                let translation = player_components.1.translation;
                player_position = Vec3::new(translation.x, translation.y, translation.z);

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
            }
            Err(_) => {
                // Soft connected chat

                let persistent_player_data_component;

                //Safety check.
                match soft_player_query.get(player_pawn_entity) {
                    Ok(_) => {}
                    Err(_rr) => {
                        continue;
                    }
                }

                match global_listeners.get(player_pawn_entity) {
                    Ok((_connected, persistent_data)) => {
                        persistent_player_data_component = persistent_data;
                    }
                    Err(_rr) => {
                        warn!("Couldnt find components for SoftConnected player with assumed global message.");
                        continue;
                    }
                }

                new_chat_message(
                    &mut net_new_chat_message_event,
                    &handle_to_entity,
                    &vec![],
                    &vec![],
                    Vec3::ZERO,
                    persistent_player_data_component.user_name.clone(),
                    ShipJobsEnum::Security,
                    chat_message_input_event.message.clone(),
                    Communicator::Standard,
                    false,
                    &radio_pawns,
                    &global_listeners,
                    Some(&player_pawn_entity),
                    Some(&mut net_send_entity_updates),
                    &MessagingPlayerState::SoftConnected,
                );
            }
        }
    }
}

pub fn net_system(
    mut net: ResMut<RenetServer>,
    connected_players: Query<&ConnectedPlayer>,
    handle_to_entity: Res<HandleToEntity>,

    mut net1: EventReader<NetChatMessage>,
) {
    for new_event in net1.iter() {
        send_net(
            &mut net,
            &connected_players,
            &handle_to_entity,
            &NetEvent {
                handle: new_event.handle,
                message: new_event.message.clone(),
            },
            RENET_RELIABLE_CHANNEL_ID,
        );
    }
}
