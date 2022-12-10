use crate::{
    boarding::{PersistentPlayerData, SoftPlayer},
    connection::{AuthidI, SendServerConfiguration},
    names::UsedNames,
};
use bevy::prelude::{Commands, Entity, EventReader, Res, ResMut, Resource};

use bevy::prelude::EventWriter;
use networking::server::GreetingClientServerMessage;

use crate::connections::PlayerServerMessage;
use networking::server::OutgoingReliableServerMessage;
use networking::server::{ConnectedPlayer, HandleToEntity};
use resources::core::{ServerId, TickRate};

/// Send server configuration to a new client that has connected.
#[cfg(feature = "server")]
pub(crate) fn server_send_configuration(
    mut config_events: EventReader<SendServerConfiguration>,
    tick_rate: Res<TickRate>,
    server_id: Res<ServerId>,
    mut auth_id_i: ResMut<AuthidI>,
    mut used_names: ResMut<UsedNames>,
    mut commands: Commands,
    mut handle_to_entity: ResMut<HandleToEntity>,
    mut server: EventWriter<OutgoingReliableServerMessage<GreetingClientServerMessage>>,
    mut server1: EventWriter<OutgoingReliableServerMessage<PlayerServerMessage>>,
) {
    for event in config_events.iter() {
        server.send(OutgoingReliableServerMessage {
            handle: event.handle,
            message: GreetingClientServerMessage::Awoo,
        });
        server1.send(OutgoingReliableServerMessage {
            handle: event.handle,
            message: PlayerServerMessage::ConfigTickRate(tick_rate.physics_rate),
        });
        server1.send(OutgoingReliableServerMessage {
            handle: event.handle,
            message: PlayerServerMessage::ConfigServerEntityId(server_id.id.to_bits()),
        });
        server1.send(OutgoingReliableServerMessage {
            handle: event.handle,
            message: PlayerServerMessage::ConfigRepeatingSFX(
                "concrete_walking_footsteps".to_string(),
                (1..=39)
                    .map(|i| {
                        format!(
                        "/content/audio/footsteps/default/Concrete_Shoes_Walking_step{i}.sample"
                    )
                    })
                    .collect(),
            ),
        });

        server1.send(OutgoingReliableServerMessage {
            handle: event.handle,
            message: PlayerServerMessage::ConfigRepeatingSFX(
                "concrete_sprinting_footsteps".to_string(),
                [
                    4, 5, 7, 9, 10, 12, 13, 14, 15, 16, 17, 20, 21, 22, 23, 24, 25, 27, 28, 30, 31,
                    32, 34, 35, 36, 38, 40, 41, 42, 43, 44, 45, 46, 47, 49, 50, 51,
                ]
                .iter()
                .map(|i| {
                    format!(
                        "/content/audio/footsteps/default/Concrete_Shoes_Running_step{i}.sample"
                    )
                })
                .collect(),
            ),
        });

        // Create the actual Bevy entity for the player , with its network handle, authid and softConnected components.

        let connected_player_component = ConnectedPlayer {
            handle: event.handle,
            authid: auth_id_i.i,
            rcon: false,
            ..Default::default()
        };

        let soft_connected_component = SoftPlayer;

        let mut default_name = "Wolf".to_string() + &used_names.player_i.to_string();

        used_names.player_i += 1;

        while used_names.account_name.contains_key(&default_name) {
            used_names.player_i += 1;
            default_name = "Wolf".to_string() + &used_names.player_i.to_string();
        }

        let persistent_player_data = PersistentPlayerData {
            character_name: "".to_string(),
            account_name: default_name.clone(),
            ..Default::default()
        };

        auth_id_i.i += 1;

        let player_entity_id = commands
            .spawn((
                connected_player_component,
                soft_connected_component,
                persistent_player_data,
            ))
            .id();
        used_names
            .account_name
            .insert(default_name, player_entity_id);

        handle_to_entity.map.insert(event.handle, player_entity_id);
        handle_to_entity
            .inv_map
            .insert(player_entity_id, event.handle);
    }
}
use crate::connections::PlayerAwaitingBoarding;
use networking::client::IncomingReliableServerMessage;

#[cfg(feature = "client")]
#[derive(Resource, Default)]
/// Stores the entity ID of the player's pawn.
pub struct PawnEntityId {
    pub option: Option<Entity>,
}
use bevy::prelude::info;

#[cfg(feature = "client")]
pub(crate) fn client_receive_pawnid(
    mut client: EventReader<IncomingReliableServerMessage<PlayerServerMessage>>,
    mut id: ResMut<PawnEntityId>,
) {
    for message in client.iter() {
        match message.message {
            PlayerServerMessage::PawnId(entity_bits) => {
                id.option = Some(Entity::from_bits(entity_bits));
                info!("Received pawn id {:?}", id.option.unwrap());
            }
            _ => (),
        }
    }
}

#[cfg(feature = "server")]
pub(crate) fn finished_configuration(
    mut config_events: EventReader<SendServerConfiguration>,
    mut server: EventWriter<OutgoingReliableServerMessage<PlayerServerMessage>>,
    mut player_awaiting_event: EventWriter<PlayerAwaitingBoarding>,
) {
    for event in config_events.iter() {
        server.send(OutgoingReliableServerMessage {
            handle: event.handle,
            message: PlayerServerMessage::ConfigFinished,
        });
        player_awaiting_event.send(PlayerAwaitingBoarding {
            handle: event.handle,
        });
    }
}
