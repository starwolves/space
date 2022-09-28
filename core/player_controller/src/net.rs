use api::{
    data::{ConnectedPlayer, GIProbe, HandleToEntity, ReflectionProbe},
    entity_updates::{EntityData, EntityUpdates},
    load_entity::{load_entity, NetLoadEntity},
    network::{PendingMessage, PendingNetworkMessage, ReliableServerMessage, ServerConfigMessage},
    player_controller::SoftPlayer,
    world_environment::WorldEnvironment,
};
use bevy::prelude::{
    Commands, Entity, EventReader, EventWriter, Local, Query, Res, Transform, Without,
};
use humanoid::humanoid::Humanoid;
use networking::messages::{InputBuildGraphics, InputMouseDirectionUpdate, InputSceneReady};
use networking_macros::NetMessage;

use std::{collections::HashMap, f32::consts::PI};

use super::connection::{Boarding, SetupPhase};
#[derive(NetMessage)]
pub(crate) struct NetSendServerTime {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

#[derive(NetMessage)]
pub(crate) struct NetSendWorldEnvironment {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
#[derive(NetMessage)]
pub(crate) struct NetUpdatePlayerCount {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
#[derive(NetMessage)]
pub(crate) struct NetDoneBoarding {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
#[derive(NetMessage)]
pub(crate) struct NetExamineEntity {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
#[derive(NetMessage)]
pub(crate) struct NetOnBoarding {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
#[derive(NetMessage)]
pub(crate) struct NetOnSetupUI {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
#[derive(NetMessage)]
pub(crate) struct NetOnSpawning {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
#[derive(NetMessage)]
pub(crate) struct NetOnNewPlayerConnection {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
#[derive(NetMessage)]
pub(crate) struct NetUserName {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

/// Build graphics for Godot client.
pub(crate) fn build_graphics(
    mut build_graphics_events: EventReader<InputBuildGraphics>,
    mut net_load_entity: EventWriter<NetLoadEntity>,
    mut net_send_world_environment: EventWriter<NetSendWorldEnvironment>,
    world_environment: Res<WorldEnvironment>,
    reflection_probe_query: Query<(
        Entity,
        &ReflectionProbe,
        &Transform,
        &EntityData,
        &EntityUpdates,
    )>,
    gi_probe_query: Query<(Entity, &GIProbe, &Transform, &EntityData, &EntityUpdates)>,
) {
    for build_graphics_event in build_graphics_events.iter() {
        net_send_world_environment.send(NetSendWorldEnvironment {
            handle: build_graphics_event.handle,
            message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::WorldEnvironment(
                *world_environment,
            )),
        });

        for (
            entity,
            _gi_probe_component,
            static_transform_component,
            entity_data_component,
            entity_updates_component,
        ) in gi_probe_query.iter()
        {
            load_entity(
                &entity_updates_component.updates,
                *static_transform_component,
                false,
                &mut net_load_entity,
                build_graphics_event.handle,
                entity_data_component,
                entity_updates_component,
                entity,
                true,
            );
        }

        for (
            entity,
            _reflection_probe_component,
            static_transform_component,
            entity_data_component,
            entity_updates_component,
        ) in reflection_probe_query.iter()
        {
            load_entity(
                &entity_updates_component.updates,
                *static_transform_component,
                false,
                &mut net_load_entity,
                build_graphics_event.handle,
                entity_data_component,
                entity_updates_component,
                entity,
                true,
            );
        }
    }
}

/// Manage when client has finished loading in a scene.
pub fn scene_ready_event(
    mut event: EventReader<InputSceneReady>,
    handle_to_entity: Res<HandleToEntity>,
    criteria_query: Query<&SoftPlayer, Without<Boarding>>,
    mut commands: Commands,
) {
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

/// Send server time to clients for ping update.
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

/// Used to calculate ping for client.
#[derive(Default)]
pub(crate) struct TimeStampPerEntity {
    pub data: HashMap<Entity, u64>,
}

/// Manage mouse direction updates.
pub(crate) fn mouse_direction_update(
    mut update_events: EventReader<InputMouseDirectionUpdate>,
    mut standard_characters: Query<&mut Humanoid>,
    mut time_stamp_per_entity: Local<TimeStampPerEntity>,
) {
    for event in update_events.iter() {
        match time_stamp_per_entity.data.get(&event.entity) {
            Some(time_stamp) => {
                if time_stamp > &event.time_stamp {
                    continue;
                }
            }
            None => {}
        }

        time_stamp_per_entity
            .data
            .insert(event.entity, event.time_stamp);

        match standard_characters.get_mut(event.entity) {
            Ok(mut standard_character_component) => {
                if standard_character_component.combat_mode == false {
                    continue;
                }

                let direction = event.direction.clamp(-PI, PI);

                standard_character_component.facing_direction = direction;
            }
            Err(_rr) => {}
        }
    }
}
