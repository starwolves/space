use bevy::prelude::{EventReader, Res};

pub struct PlayerAwaitingBoarding {
    pub handle: u64,
}
use bevy::prelude::info;
use bevy::prelude::warn;
use bevy_renet::renet::RenetServer;
use bevy_renet::renet::ServerEvent;

/// Networking connect and disconnect events.

pub(crate) fn server_events(mut server_events: EventReader<ServerEvent>, server: Res<RenetServer>) {
    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected(handle, token) => {
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
use bevy::prelude::Component;
use bevy::prelude::Resource;

/// The component for entities int he boarding phase.
#[derive(Component)]

pub struct SetupPhase;

/// The component for entities that are done boarding and about to spawn in on the ship. A stage after [Boarding].
#[derive(Component)]

pub struct OnBoard;

/// Event for sending server configuration to newly connected client. Done after client account is verified.

pub struct SendServerConfiguration {
    pub handle: u64,
}
/// Resource with the current incremented authentication ID.
#[derive(Default, Resource)]

pub(crate) struct AuthidI {
    pub i: u16,
}
