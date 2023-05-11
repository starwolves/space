use bevy::prelude::{
    error, Commands, DespawnRecursiveExt, Entity, EventReader, Query, Res, ResMut,
};

pub struct PlayerAwaitingBoarding {
    pub handle: u64,
}
use bevy::prelude::info;
use bevy::prelude::warn;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use bevy_renet::renet::RenetServer;
use bevy_renet::renet::ServerEvent;

/// Networking connect and disconnect events.

#[derive(Component)]
pub struct VerifyToken {
    pub task: Task<ehttp::Response>,
    pub handle: u64,
}

pub(crate) fn server_events(
    mut server_events: EventReader<ServerEvent>,
    server: Res<RenetServer>,
    mut commands: Commands,
) {
    for event in server_events.iter() {
        let server_addr = local_ipaddress::get().unwrap_or_default();

        match event {
            ServerEvent::ClientConnected(handle, header) => {
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

                let token;

                match String::from_utf8((**header).to_vec()) {
                    Ok(t) => {
                        token = t;
                    }
                    Err(rr) => {
                        warn!("Couldn't decode token from [{}]: {}", handle, rr);
                        continue;
                    }
                }

                let client_ip = client_address.ip().to_string();

                let is_local = server_addr == client_ip;

                info!("Incoming connection [{}] [{:?}]", handle, client_address);
                let data = vec![
                    ("token", token),
                    ("userAddress", client_ip),
                    ("isLocal", is_local.to_string()),
                ];

                let x = VerifyToken {
                    task: AsyncComputeTaskPool::get().spawn(async move {
                        let encoded = form_urlencoded::Serializer::new(String::new())
                            .extend_pairs(data)
                            .finish();

                        let mut post = ehttp::Request::post(
                            format!("https://store.starwolves.io/server_token_verify"),
                            encoded.into_bytes(),
                        );
                        post.headers = ehttp::headers(&[
                            ("Accept", "*/*"),
                            (
                                "Content-Type",
                                "application/x-www-form-urlencoded; charset=utf-8",
                            ),
                        ]);
                        ehttp::fetch_blocking(&post).expect("Error with HTTP call")
                    }),
                    handle: *handle,
                };

                commands.spawn(x);
            }
            ServerEvent::ClientDisconnected(handle) => {
                info!("[{}] has disconnected.", handle);
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Response {
    pub valid: bool,
    pub name: String,
}

pub fn process_response(
    mut commands: Commands,
    mut query: Query<(Entity, &mut VerifyToken)>,
    mut server: ResMut<RenetServer>,
) {
    for (entity, mut token) in query.iter_mut() {
        if let Some(response) = future::block_on(future::poll_once(&mut token.task)) {
            // Process the response
            match serde_json::from_slice::<Response>(response.bytes.as_slice()) {
                Ok(d) => {
                    if !d.valid {
                        warn!(
                            "Invalid token received from [{}]. Disconnecting..",
                            token.handle
                        );
                        server.disconnect(token.handle);
                    } else {
                        info!("[Starwolves.io] Successfully verified [{}]", token.handle);
                    }

                    commands.entity(entity).despawn_recursive();
                }
                Err(e) => {
                    error!("Unexpected response: {:?}", e,);
                }
            }
        }
    }
}

use bevy::prelude::Component;
use bevy::prelude::Resource;
use futures_lite::future;
use serde::{Deserialize, Serialize};

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
