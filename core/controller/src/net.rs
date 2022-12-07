use crate::input::InputBuildGraphics;
use bevy::prelude::With;
use bevy::prelude::{Entity, EventReader, EventWriter, Query, Res};
use entity::networking::LoadEntity;
use gi_probe::core::GIProbe;
use networking::server::OutgoingReliableServerMessage;
use reflection_probe::core::ReflectionProbe;
use world_environment::environment::WorldEnvironment;

use player::connections::PlayerServerMessage;

/// Build graphics for Godot client.
#[cfg(feature = "server")]
pub(crate) fn build_graphics(
    mut build_graphics_events: EventReader<InputBuildGraphics>,
    mut server: EventWriter<OutgoingReliableServerMessage<PlayerServerMessage>>,
    world_environment: Res<WorldEnvironment>,
    reflection_probe_query: Query<Entity, With<ReflectionProbe>>,
    gi_probe_query: Query<Entity, With<GIProbe>>,
    mut load_entity_event: EventWriter<LoadEntity>,
) {
    for build_graphics_event in build_graphics_events.iter() {
        server.send(OutgoingReliableServerMessage {
            handle: build_graphics_event.handle,
            message: PlayerServerMessage::ConfigWorldEnvironment(*world_environment),
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

use networking::server::ConnectedPlayer;

/// Send server time to clients for ping update.
#[cfg(feature = "server")]
pub(crate) fn send_server_time(
    mut server: EventWriter<OutgoingReliableServerMessage<PlayerServerMessage>>,
    connected_players: Query<&ConnectedPlayer>,
) {
    for connected_player_component in connected_players.iter() {
        if !connected_player_component.connected {
            continue;
        }

        server.send(OutgoingReliableServerMessage {
            handle: connected_player_component.handle,
            message: PlayerServerMessage::ServerTime,
        });
    }
}

/// Update player count info for clients.
#[cfg(feature = "server")]
pub(crate) fn update_player_count(
    connected_players: Query<&ConnectedPlayer>,
    mut server: EventWriter<OutgoingReliableServerMessage<PlayerServerMessage>>,
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
        server.send(OutgoingReliableServerMessage {
            handle: connected_player_component.handle,
            message: PlayerServerMessage::ConnectedPlayers(connected_players_amount),
        });
    }
}
