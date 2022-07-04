use bevy::prelude::{
    Commands, Entity, EventReader, EventWriter, Local, Query, Res, ResMut, Transform, Without,
};
use bevy_renet::renet::RenetServer;

pub struct NetSendServerTime {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

pub struct NetSendWorldEnvironment {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

pub struct NetUpdatePlayerCount {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

pub struct NetDoneBoarding {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

pub struct NetExamineEntity {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

pub struct NetOnBoarding {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

pub struct NetOnNewPlayerConnection {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

pub struct NetOnSetupUI {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

pub struct NetTabData {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

pub struct NetUIInputTransmitData {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

pub struct NetUserName {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

pub struct NetOnSpawning {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

pub fn net_system(
    mut net: ResMut<RenetServer>,
    connected_players: Query<&ConnectedPlayer>,
    handle_to_entity: Res<HandleToEntity>,
    mut net1: EventReader<NetOnBoarding>,
    mut net2: EventReader<NetOnNewPlayerConnection>,
    mut net3: EventReader<NetOnSetupUI>,
    mut net4: EventReader<NetDoneBoarding>,
    mut net5: EventReader<NetSendWorldEnvironment>,
    mut net6: EventReader<NetOnSpawning>,
    mut net7: EventReader<NetUserName>,
    mut net8: EventReader<NetUIInputTransmitData>,
    mut net9: EventReader<NetExamineEntity>,
    mut net10: EventReader<NetTabData>,
    mut net11: EventReader<NetSendServerTime>,
    mut net12: EventReader<NetUpdatePlayerCount>,
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
        );
    }
    for new_event in net2.iter() {
        send_net(
            &mut net,
            &connected_players,
            &handle_to_entity,
            &NetEvent {
                handle: new_event.handle,
                message: new_event.message.clone(),
            },
        );
    }
    for new_event in net3.iter() {
        send_net(
            &mut net,
            &connected_players,
            &handle_to_entity,
            &NetEvent {
                handle: new_event.handle,
                message: new_event.message.clone(),
            },
        );
    }
    for new_event in net4.iter() {
        send_net(
            &mut net,
            &connected_players,
            &handle_to_entity,
            &NetEvent {
                handle: new_event.handle,
                message: new_event.message.clone(),
            },
        );
    }
    for new_event in net5.iter() {
        send_net(
            &mut net,
            &connected_players,
            &handle_to_entity,
            &NetEvent {
                handle: new_event.handle,
                message: new_event.message.clone(),
            },
        );
    }
    for new_event in net6.iter() {
        send_net(
            &mut net,
            &connected_players,
            &handle_to_entity,
            &NetEvent {
                handle: new_event.handle,
                message: new_event.message.clone(),
            },
        );
    }
    for new_event in net7.iter() {
        send_net(
            &mut net,
            &connected_players,
            &handle_to_entity,
            &NetEvent {
                handle: new_event.handle,
                message: new_event.message.clone(),
            },
        );
    }
    for new_event in net8.iter() {
        send_net(
            &mut net,
            &connected_players,
            &handle_to_entity,
            &NetEvent {
                handle: new_event.handle,
                message: new_event.message.clone(),
            },
        );
    }
    for new_event in net9.iter() {
        send_net(
            &mut net,
            &connected_players,
            &handle_to_entity,
            &NetEvent {
                handle: new_event.handle,
                message: new_event.message.clone(),
            },
        );
    }
    for new_event in net10.iter() {
        send_net(
            &mut net,
            &connected_players,
            &handle_to_entity,
            &NetEvent {
                handle: new_event.handle,
                message: new_event.message.clone(),
            },
        );
    }
    for new_event in net11.iter() {
        send_net(
            &mut net,
            &connected_players,
            &handle_to_entity,
            &NetEvent {
                handle: new_event.handle,
                message: new_event.message.clone(),
            },
        );
    }
    for new_event in net12.iter() {
        send_net(
            &mut net,
            &connected_players,
            &handle_to_entity,
            &NetEvent {
                handle: new_event.handle,
                message: new_event.message.clone(),
            },
        );
    }
}

pub struct InputBuildGraphics {
    pub handle: u64,
}

use crate::{
    core::{
        entity::{
            entity_data::EntityData,
            entity_updates::EntityUpdates,
            load_entity::{load_entity, NetLoadEntity},
        },
        humanoid::humanoid::Humanoid,
        networking::{
            net::send_net,
            networking::{ReliableServerMessage, ServerConfigMessage},
            plugin::NetEvent,
        },
        world_environment::environment::WorldEnvironment,
    },
    entities::{gi_probe::spawn::GIProbe, reflection_probe::reflection_probe::ReflectionProbe},
};

pub fn build_graphics_event(
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

pub struct InputSceneReady {
    pub handle: u64,
    pub scene_type: String,
}

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

        if new_event.scene_type == "setupUI" {
            commands.entity(*player_entity).insert(SetupPhase);
        }
    }
}

pub fn send_server_time(
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

pub fn update_player_count(
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

use std::{collections::HashMap, f32::consts::PI};

pub struct InputMouseDirectionUpdate {
    pub entity: Entity,
    pub direction: f32,
    pub time_stamp: u64,
}

use super::{
    connection::{Boarding, ConnectedPlayer, SetupPhase, SoftPlayer},
    plugin::HandleToEntity,
};

#[derive(Default)]
pub struct TimeStampPerEntity {
    pub data: HashMap<Entity, u64>,
}

pub fn mouse_direction_update(
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
