use bevy::prelude::{Commands, EventReader, EventWriter, Res, ResMut};
use networking::server::{ConnectedPlayer, HandleToEntity};
use resources::core::{ServerId, TickRate};
use serde::{Deserialize, Serialize};
use typename::TypeName;
use world_environment::environment::WorldEnvironment;

use crate::{
    boarding::{PersistentPlayerData, SoftPlayer},
    connection::{AuthidI, SendServerConfiguration},
    names::UsedNames,
};
use networking::server::NetworkingClientServerMessage;
use networking::typenames::OutgoingReliableServerMessage;

#[cfg(feature = "server")]
pub(crate) fn configure(
    mut config_events: EventReader<SendServerConfiguration>,
    tick_rate: Res<TickRate>,
    server_id: Res<ServerId>,
    mut auth_id_i: ResMut<AuthidI>,
    mut used_names: ResMut<UsedNames>,
    mut commands: Commands,
    mut handle_to_entity: ResMut<HandleToEntity>,
    mut server: EventWriter<OutgoingReliableServerMessage<NetworkingClientServerMessage>>,
    mut server1: EventWriter<OutgoingReliableServerMessage<PlayerServerMessage>>,
) {
    for event in config_events.iter() {
        server.send(OutgoingReliableServerMessage {
            handle: event.handle,
            message: NetworkingClientServerMessage::Awoo,
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

        server1.send(OutgoingReliableServerMessage {
            handle: event.handle,
            message: PlayerServerMessage::ConfigEntityId(player_entity_id.to_bits()),
        });
    }
}

pub struct PlayerAwaitingBoarding {
    pub handle: u64,
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
use bevy::prelude::info;
use bevy::prelude::warn;
use bevy_renet::renet::RenetServer;
use bevy_renet::renet::ServerEvent;

#[cfg(feature = "server")]
pub(crate) fn server_events(
    mut server_events: EventReader<ServerEvent>,
    server: Res<RenetServer>,
    mut configure: EventWriter<SendServerConfiguration>,
) {
    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected(handle, _) => {
                let client_address;

                match server.client_addr(*handle) {
                    Some(ip) => {
                        client_address = ip;
                    }
                    None => {
                        warn!("Couldn't get address from [{}]", handle);
                        continue;
                    }
                };

                info!("Incoming connection [{}] [{:?}]", handle, client_address);
                configure.send(SendServerConfiguration { handle: *handle })
            }
            ServerEvent::ClientDisconnected(handle) => {
                info!("[{}] has disconnected.", handle);
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum PlayerServerMessage {
    InitSetupUi,
    InitGame,
    ConfigWorldEnvironment(WorldEnvironment),
    ServerTime,
    ConnectedPlayers(u16),
    ConfigTickRate(u8),
    ConfigEntityId(u64),
    ConfigServerEntityId(u64),
    ConfigRepeatingSFX(String, Vec<String>),
    ConfigFinished,
    ConfigTalkSpaces(Vec<(String, String)>),
}
use networking::client::Connection;
use networking::client::ConnectionStatus;
use networking::typenames::get_reliable_message;
use networking::typenames::IncomingReliableServerMessage;
use networking::typenames::Typenames;

/// Confirms connection with server.
#[cfg(feature = "client")]
pub(crate) fn confirm_connection(
    mut client: EventReader<IncomingReliableServerMessage>,
    mut connected_state: ResMut<Connection>,
    typenames: Res<Typenames>,
) {
    for message in client.iter() {
        match get_reliable_message::<NetworkingClientServerMessage>(
            &typenames,
            message.message.typename_net,
            &message.message.serialized,
        ) {
            Some(player_message) => match player_message {
                NetworkingClientServerMessage::Awoo => {
                    connected_state.status = ConnectionStatus::Connected;
                    info!("Connected.");
                }
            },
            None => {}
        }
    }
}
