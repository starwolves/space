use bevy::prelude::{Commands, EventReader, EventWriter, Res, ResMut};
use networking::server::{
    ConnectedPlayer, HandleToEntity, ReliableServerMessage, ServerConfigMessage,
};
use resources::core::{ServerId, TickRate};

use crate::{
    boarding::{PersistentPlayerData, SoftPlayer},
    connection::{AuthidI, NetPlayerConn, SendServerConfiguration},
    names::UsedNames,
};
#[cfg(feature = "server")]
pub(crate) fn configure(
    mut config_events: EventReader<SendServerConfiguration>,
    mut net_on_new_player_connection: EventWriter<NetPlayerConn>,
    tick_rate: Res<TickRate>,
    server_id: Res<ServerId>,
    mut auth_id_i: ResMut<AuthidI>,
    mut used_names: ResMut<UsedNames>,
    mut commands: Commands,
    mut handle_to_entity: ResMut<HandleToEntity>,
) {
    for event in config_events.iter() {
        net_on_new_player_connection.send(NetPlayerConn {
            handle: event.handle,
            message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::Awoo),
        });

        net_on_new_player_connection.send(NetPlayerConn {
            handle: event.handle,
            message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::TickRate(
                tick_rate.physics_rate,
            )),
        });

        net_on_new_player_connection.send(NetPlayerConn {
            handle: event.handle,
            message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::ServerEntityId(
                server_id.id.to_bits(),
            )),
        });

        net_on_new_player_connection.send(NetPlayerConn {
            handle: event.handle,
            message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::ChangeScene(
                false,
                "setupUI".to_string(),
            )),
        });

        net_on_new_player_connection.send(NetPlayerConn {
            handle: event.handle,
            message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::RepeatingSFX(
                "concrete_walking_footsteps".to_string(),
                (1..=39)
                    .map(|i| {
                        format!(
                        "/content/audio/footsteps/default/Concrete_Shoes_Walking_step{i}.sample"
                    )
                    })
                    .collect(),
            )),
        });

        net_on_new_player_connection.send(NetPlayerConn {
            handle: event.handle,
            message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::RepeatingSFX(
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
            )),
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

        net_on_new_player_connection.send(NetPlayerConn {
            handle: event.handle,
            message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::EntityId(
                player_entity_id.to_bits(),
            )),
        });
    }
}

pub(crate) fn finished_configuration(
    mut config_events: EventReader<SendServerConfiguration>,
    mut net_on_new_player_connection: EventWriter<NetPlayerConn>,
) {
    for event in config_events.iter() {
        net_on_new_player_connection.send(NetPlayerConn {
            handle: event.handle,
            message: ReliableServerMessage::ConfigMessage(
                ServerConfigMessage::FinishedInitialization,
            ),
        });
    }
}
