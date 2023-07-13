use std::collections::HashMap;
use std::net::SocketAddr;

use bevy::prelude::{
    error, Commands, DespawnRecursiveExt, Entity, Event, EventReader, EventWriter, Query, Res,
    ResMut,
};
#[derive(Event)]
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
    mut commands: Commands,
    server_data: Res<NetcodeServerTransport>,
) {
    for event in server_events.iter() {
        let server_addr = local_ipaddress::get().unwrap_or_default();

        match event {
            ServerEvent::ClientConnected { client_id } => {
                let client_address;
                client_address = SocketAddr::new(
                    local_ipaddress::get().unwrap_or_default().parse().unwrap(),
                    57713,
                );
                /*match server_data.netcode_server.client_addr(*client_id) {
                    Ok(info) => {
                        client_address = ip;
                    }
                    Err(err) => {
                        warn!("Couldn't get address from [{}]: {}", client_id, err);
                        continue;
                    }
                };*/

                let client_ip = client_address.ip().to_string();

                let raw_token;
                match server_data.user_data(*client_id) {
                    Some(r) => {
                        raw_token = r;
                    }
                    None => {
                        warn!("Couldnt get user data.");
                        continue;
                    }
                }

                let token;

                match String::from_utf8((raw_token).to_vec()) {
                    Ok(t) => {
                        token = t;
                    }
                    Err(rr) => {
                        warn!("Couldn't decode token from [{}]: {}", client_id, rr);
                        continue;
                    }
                }

                let is_local = server_addr == client_ip;

                info!("Incoming connection [{}] [{:?}]", client_id, client_address);
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
                    handle: *client_id,
                };

                commands.spawn(x);
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                info!("[{}] has disconnected: {}.", client_id, reason);
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Response {
    pub valid: bool,
    pub name: String,
}
/// Player accounts stored with handles.
#[derive(Default, Resource)]

pub struct Accounts {
    pub list: HashMap<u64, String>,
}
pub fn process_response(
    mut commands: Commands,
    mut query: Query<(Entity, &mut VerifyToken)>,
    mut server: ResMut<RenetServer>,
    mut accounts: ResMut<Accounts>,
    mut used_names: ResMut<UsedNames>,
    mut outgoing: EventWriter<OutgoingReliableServerMessage<NetworkingServerMessage>>,
    mut configure: EventWriter<SendServerConfiguration>,
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

                        used_names.used_account_names.push(d.name.clone());
                        accounts.list.insert(token.handle, d.name.clone());

                        outgoing.send(OutgoingReliableServerMessage {
                            handle: token.handle,
                            message: NetworkingServerMessage::Awoo,
                        });

                        configure.send(SendServerConfiguration {
                            handle: token.handle,
                        });

                        info!("Set account name {} for {}", d.name, token.handle);
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
use bevy_renet::renet::transport::NetcodeServerTransport;
use futures_lite::future;
use networking::server::{NetworkingServerMessage, OutgoingReliableServerMessage};
use serde::{Deserialize, Serialize};

use crate::names::UsedNames;

/// The component for entities int he boarding phase.
#[derive(Component)]

pub struct SetupPhase;

/// The component for entities that are done boarding and about to spawn in on the ship. A stage after [Boarding].
#[derive(Component)]

pub struct OnBoard;

/// Event for sending server configuration to newly connected client. Done after client account is verified.
#[derive(Event)]
pub struct SendServerConfiguration {
    pub handle: u64,
}
/// Resource with the current incremented authentication ID.
#[derive(Default, Resource)]

pub(crate) struct AuthidI {
    pub i: u16,
}
