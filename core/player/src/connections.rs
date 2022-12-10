use bevy::prelude::{EventReader, Res};

use serde::{Deserialize, Serialize};
use typename::TypeName;
use world_environment::environment::WorldEnvironment;

pub struct PlayerAwaitingBoarding {
    pub handle: u64,
}
use bevy::prelude::info;
use bevy::prelude::warn;
use bevy_renet::renet::RenetServer;
use bevy_renet::renet::ServerEvent;

/// Networking connect and disconnect events.
#[cfg(feature = "server")]
pub(crate) fn server_events(mut server_events: EventReader<ServerEvent>, server: Res<RenetServer>) {
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
    InitGame,
    ConfigWorldEnvironment(WorldEnvironment),
    ServerTime,
    ConnectedPlayers(u16),
    ConfigTickRate(u8),
    PawnId(u64),
    ConfigServerEntityId(u64),
    ConfigRepeatingSFX(String, Vec<String>),
    ConfigFinished,
    ConfigTalkSpaces(Vec<(String, String)>),
}
use bevy::prelude::Component;
use bevy::prelude::Resource;

/// The component for entities int he boarding phase.
#[derive(Component)]
#[cfg(feature = "server")]
pub struct SetupPhase;

/// The component for entities that are done boarding and about to spawn in on the ship. A stage after [Boarding].
#[derive(Component)]
#[cfg(feature = "server")]
pub struct OnBoard;

/// Event for sending server configuration to newly connected client. Done after client account is verified.
#[cfg(feature = "server")]
pub struct SendServerConfiguration {
    pub handle: u64,
}
/// Resource with the current incremented authentication ID.
#[derive(Default, Resource)]
#[cfg(feature = "server")]
pub(crate) struct AuthidI {
    pub i: u16,
}
