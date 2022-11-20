use bevy::prelude::{Commands, Entity, EventReader, EventWriter, Query, Res, Without};
use gi_probe::core::GIProbe;
use networking::server::{ReliableServerMessage, ServerConfigMessage};
use networking_macros::NetMessage;
use reflection_probe::core::ReflectionProbe;
use world_environment::environment::WorldEnvironment;

use networking::server::PendingMessage;
use networking::server::PendingNetworkMessage;
#[derive(NetMessage)]
#[cfg(feature = "server")]
pub(crate) struct NetSendServerTime {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

#[derive(NetMessage)]
#[cfg(feature = "server")]
pub(crate) struct NetSendWorldEnvironment {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
#[derive(NetMessage)]
#[cfg(feature = "server")]
pub(crate) struct NetUpdatePlayerCount {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

#[derive(NetMessage)]
#[cfg(feature = "server")]
pub(crate) struct NetExamineEntity {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
#[derive(NetMessage)]
#[cfg(feature = "server")]
pub(crate) struct NetOnNewPlayerConnection {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
#[derive(NetMessage)]
#[cfg(feature = "server")]
pub(crate) struct NetUserName {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
use crate::input::InputBuildGraphics;
use bevy::prelude::With;
use entity::networking::LoadEntity;

/// Build graphics for Godot client.
#[cfg(feature = "server")]
pub(crate) fn build_graphics(
    mut build_graphics_events: EventReader<InputBuildGraphics>,
    mut net_send_world_environment: EventWriter<NetSendWorldEnvironment>,
    world_environment: Res<WorldEnvironment>,
    reflection_probe_query: Query<Entity, With<ReflectionProbe>>,
    gi_probe_query: Query<Entity, With<GIProbe>>,
    mut load_entity_event: EventWriter<LoadEntity>,
) {
    for build_graphics_event in build_graphics_events.iter() {
        net_send_world_environment.send(NetSendWorldEnvironment {
            handle: build_graphics_event.handle,
            message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::WorldEnvironment(
                *world_environment,
            )),
        });

        for entity in gi_probe_query.iter() {
            load_entity_event.send(LoadEntity {
                entity: entity,
                loader_handle: build_graphics_event.handle,
                load_entirely: true,
            });
        }

        for entity in reflection_probe_query.iter() {
            load_entity_event.send(LoadEntity {
                entity: entity,
                loader_handle: build_graphics_event.handle,
                load_entirely: true,
            });
        }
    }
}
use crate::input::InputSceneReady;

use networking::server::HandleToEntity;
use player::{boarding::SoftPlayer, connection::Boarding};
/// Manage when client has finished loading in a scene.
#[cfg(feature = "server")]
pub fn scene_ready_event(
    mut event: EventReader<InputSceneReady>,
    handle_to_entity: Res<HandleToEntity>,
    criteria_query: Query<&SoftPlayer, Without<Boarding>>,
    mut commands: Commands,
) {
    use player::connection::SetupPhase;

    for new_event in event.iter() {
        let player_entity = handle_to_entity.map.get(&new_event.handle)
        .expect("scene_ready_event.rs could not find components for player that just got done boarding.");

        //Safety check.
        match criteria_query.get(*player_entity) {
            Ok(_) => {}
            Err(_rr) => {
                continue;
            }
        }

        if new_event.scene_id == "setupUI" {
            commands.entity(*player_entity).insert(SetupPhase);
        }
    }
}
use networking::server::ConnectedPlayer;

/// Send server time to clients for ping update.
#[cfg(feature = "server")]
pub(crate) fn send_server_time(
    mut event_writer: EventWriter<NetSendServerTime>,
    connected_players: Query<&ConnectedPlayer>,
) {
    for connected_player_component in connected_players.iter() {
        if !connected_player_component.connected {
            continue;
        }

        event_writer.send(NetSendServerTime {
            handle: connected_player_component.handle,
            message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::ServerTime),
        });
    }
}

/// Update player count info for clients.
#[cfg(feature = "server")]
pub(crate) fn update_player_count(
    connected_players: Query<&ConnectedPlayer>,
    mut events: EventWriter<NetUpdatePlayerCount>,
) {
    let mut connected_players_amount: u16 = 0;

    for connected_player_component in connected_players.iter() {
        if connected_player_component.connected {
            connected_players_amount += 1;
        }
    }

    for connected_player_component in connected_players.iter() {
        if !connected_player_component.connected {
            continue;
        }

        events.send(NetUpdatePlayerCount {
            handle: connected_player_component.handle,
            message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::ConnectedPlayers(
                connected_players_amount,
            )),
        });
    }
}
