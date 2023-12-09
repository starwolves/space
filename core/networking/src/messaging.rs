use std::collections::HashMap;

use bevy::prelude::{resource_exists, App, FixedUpdate, IntoSystemConfigs, Startup, SystemSet};
use bevy::prelude::{ResMut, Resource};
use bevy_renet::renet::RenetClient;
use resources::sets::MainSet;
use typename::TypeName;

/// Resource containing typenames and smaller 16-bit netcode representations. Needed to identify Rust types sent over the net.

#[derive(Resource, Default)]
pub struct Typenames {
    pub reliable_incremental_id: u16,
    pub unreliable_incremental_id: u8,
    pub reliable_types: Vec<String>,
    pub reliable_unordered_types: Vec<String>,
    pub unreliable_types: Vec<String>,
    pub reliable_net_types: HashMap<String, u16>,
    pub reliable_unordered_net_types: HashMap<String, u16>,
    pub unreliable_net_types: HashMap<String, u8>,
}

use bevy::log::warn;

/// Generic startup system that registers reliable netcode message types. All reliable netcode types sent over the net must be registered with this system.

pub(crate) fn reliable_message<T: TypeName>(mut typenames: ResMut<Typenames>) {
    typenames.reliable_types.push(T::type_name());
}
pub(crate) fn reliable_unordered_message<T: TypeName>(mut typenames: ResMut<Typenames>) {
    typenames.reliable_unordered_types.push(T::type_name());
}
/// Generic startup system that registers unreliable netcode message types. All unreliable netcode types sent over the net must be registered with this system.

pub(crate) fn unreliable_message<T: TypeName>(mut typenames: ResMut<Typenames>) {
    typenames.unreliable_types.push(T::type_name());
}
use bevy::log::info;

/// Order and generate typenames.

pub fn generate_typenames(mut typenames: ResMut<Typenames>) {
    let mut r_iter = typenames.reliable_types.clone();
    r_iter.sort();
    for typename in r_iter {
        typenames.reliable_types.push(typename.clone());
        let i = typenames.reliable_incremental_id;
        typenames.reliable_net_types.insert(typename, i);
        typenames.reliable_incremental_id += 1;

        if typenames.reliable_incremental_id >= u16::MAX {
            warn!("Reached maximum number of reliable serializable netcode messages.");
        }
    }
    let mut u_iter = typenames.unreliable_types.clone();
    u_iter.sort();
    for typename in u_iter {
        typenames.unreliable_types.push(typename.clone());
        let i = typenames.unreliable_incremental_id;
        typenames.unreliable_net_types.insert(typename, i);
        typenames.unreliable_incremental_id += 1;

        if typenames.unreliable_incremental_id >= u8::MAX {
            warn!("Reached maximum number of unreliable serializable netcode messages.");
        }
    }
    info!(
        "Loaded {} serializable messages.",
        typenames.reliable_net_types.len() + typenames.unreliable_net_types.len()
    );
}

pub enum MessageSender {
    Client,
    Server,
    Both,
}

use crate::client::step_buffer;
use crate::server::{
    clear_entity_updates, serialize_reliable_entity_updates, serialize_unreliable_entity_updates,
    EntityUpdates, EntityUpdatesSet, IncomingEarlyReliableClientMessage,
    IncomingEarlyUnreliableClientMessage, ServerMessageSet,
};
use crate::{
    client::{
        deserialize_incoming_reliable_server_message, send_outgoing_reliable_client_messages,
        IncomingReliableServerMessage, OutgoingReliableClientMessage,
    },
    server::{
        deserialize_incoming_reliable_client_message, send_outgoing_reliable_server_messages,
        IncomingReliableClientMessage, OutgoingReliableServerMessage,
    },
};
use std::fmt::Debug;
/// All reliable networking messages must be registered with this system.

pub fn register_reliable_message<
    T: Debug + TypeName + Send + Sync + Clone + Serialize + for<'a> Deserialize<'a> + 'static,
>(
    app: &mut App,
    sender: MessageSender,
    ordered: bool,
) {
    app.add_systems(
        Startup,
        reliable_message::<T>.in_set(TypenamesSet::Generate),
    );

    if !ordered {
        app.add_systems(
            Startup,
            reliable_unordered_message::<T>.in_set(TypenamesSet::Generate),
        );
    }

    let mut client_is_sender = false;
    let mut server_is_sender = false;

    match sender {
        MessageSender::Client => {
            client_is_sender = true;
        }
        MessageSender::Server => {
            server_is_sender = true;
        }
        MessageSender::Both => {
            client_is_sender = true;
            server_is_sender = true;
        }
    }

    app.add_event::<OutgoingReliableServerMessage<T>>();
    if server_is_sender && is_server() && !is_correction_mode(app) {
        app.add_systems(
            FixedUpdate,
            (
                send_outgoing_reliable_server_messages::<T>
                    .in_set(MainSet::PostUpdate)
                    .in_set(ServerMessageSet::Send),
                clear_entity_updates::<T>.in_set(MainSet::PreUpdate),
                serialize_reliable_entity_updates::<T>
                    .in_set(MainSet::PostUpdate)
                    .in_set(EntityUpdatesSet::Serialize),
            ),
        )
        .insert_resource(EntityUpdates::<T> {
            map: HashMap::default(),
        });
    }
    app.add_event::<IncomingReliableServerMessage<T>>();
    if server_is_sender && !is_server_mode(app) {
        app.add_systems(
            FixedUpdate,
            deserialize_incoming_reliable_server_message::<T>
                .after(TypenamesSet::SendRawEvents)
                .in_set(MessagingSet::DeserializeIncoming)
                .in_set(MainSet::PreUpdate),
        );
    }
    app.add_event::<OutgoingReliableClientMessage<T>>();

    if client_is_sender && !is_server_mode(app) {
        app.add_systems(
            FixedUpdate,
            send_outgoing_reliable_client_messages::<T>
                .in_set(MainSet::PostUpdate)
                .before(step_buffer),
        );
    }
    app.add_event::<IncomingReliableClientMessage<T>>()
        .add_event::<IncomingEarlyReliableClientMessage<T>>();

    if client_is_sender && is_server_mode(app) {
        app.add_systems(
            FixedUpdate,
            deserialize_incoming_reliable_client_message::<T>
                .after(TypenamesSet::SendRawEvents)
                .in_set(MainSet::PreUpdate)
                .in_set(MessagingSet::DeserializeIncoming),
        );
    }
}
use resources::modes::{is_correction_mode, is_server, is_server_mode};

/// All unreliable networking messages must be registered with this system.
pub fn register_unreliable_message<
    T: TypeName + Send + Sync + Serialize + Clone + for<'a> Deserialize<'a> + 'static,
>(
    app: &mut App,
    sender: MessageSender,
) {
    use crate::{
        client::{
            deserialize_incoming_unreliable_server_message,
            send_outgoing_unreliable_client_messages, IncomingUnreliableServerMessage,
            OutgoingUnreliableClientMessage,
        },
        server::{
            deserialize_incoming_unreliable_client_message,
            send_outgoing_unreliable_server_messages, IncomingUnreliableClientMessage,
            OutgoingUnreliableServerMessage,
        },
    };

    app.add_systems(
        Startup,
        unreliable_message::<T>.in_set(TypenamesSet::Generate),
    );
    let mut client_is_sender = false;
    let mut server_is_sender = false;

    match sender {
        MessageSender::Client => {
            client_is_sender = true;
        }
        MessageSender::Server => {
            server_is_sender = true;
        }
        MessageSender::Both => {
            client_is_sender = true;
            server_is_sender = true;
        }
    }
    if server_is_sender && is_server_mode(app) {
        app.add_event::<OutgoingUnreliableServerMessage<T>>();
        if !is_correction_mode(app) {
            app.add_systems(
                FixedUpdate,
                (
                    send_outgoing_unreliable_server_messages::<T>
                        .in_set(ServerMessageSet::Send)
                        .in_set(MainSet::PostUpdate),
                    clear_entity_updates::<T>.in_set(MainSet::PreUpdate),
                    serialize_unreliable_entity_updates::<T>
                        .in_set(MainSet::PostUpdate)
                        .in_set(EntityUpdatesSet::Serialize),
                ),
            )
            .insert_resource(EntityUpdates::<T> {
                map: HashMap::default(),
            });
        }
    }
    if server_is_sender && !is_server_mode(app) {
        app.add_event::<IncomingUnreliableServerMessage<T>>()
            .add_systems(
                FixedUpdate,
                deserialize_incoming_unreliable_server_message::<T>
                    .after(TypenamesSet::SendRawEvents)
                    .in_set(MessagingSet::DeserializeIncoming)
                    .in_set(MainSet::PreUpdate),
            );
    }
    if client_is_sender && !is_server_mode(app) {
        app.add_event::<OutgoingUnreliableClientMessage<T>>()
            .add_systems(
                FixedUpdate,
                send_outgoing_unreliable_client_messages::<T>
                    .in_set(MainSet::PostUpdate)
                    .run_if(resource_exists::<RenetClient>())
                    .before(step_buffer),
            );
    }
    if client_is_sender && is_server_mode(app) {
        app.add_event::<IncomingUnreliableClientMessage<T>>()
            .add_event::<IncomingEarlyUnreliableClientMessage<T>>()
            .add_systems(
                FixedUpdate,
                deserialize_incoming_unreliable_client_message::<T>
                    .after(TypenamesSet::SendRawEvents)
                    .in_set(MainSet::PreUpdate)
                    .in_set(MessagingSet::DeserializeIncoming),
            );
    }
}

/// Wrapper for reliable server messages.
#[derive(Serialize, Deserialize, Clone)]
pub struct ReliableMessage {
    // The message.
    pub serialized: Vec<u8>,
    // The message type.
    pub typename_net: u16,
}

/// Batch of reliable server messages.
#[derive(Serialize, Deserialize, Clone)]
pub struct ReliableServerMessageBatch {
    // The messages of this batch.
    pub messages: Vec<ReliableMessage>,
    // The confirmed tick stamp.
    pub stamp: u8,
}

/// Batch of reliable client messages.
#[derive(Serialize, Deserialize, Clone)]
pub struct ReliableClientMessageBatch {
    pub messages: Vec<ReliableMessage>,
    pub stamp: u8,
    pub sub_step: bool,
}
/// Batch of unreliable messages.
#[derive(Serialize, Deserialize, Clone)]
pub struct UnreliableServerMessageBatch {
    pub messages: Vec<UnreliableMessage>,
    pub stamp: u8,
}
/// Batch of unreliable messages.
#[derive(Serialize, Deserialize, Clone)]
pub struct UnreliableClientMessageBatch {
    pub messages: Vec<UnreliableMessage>,
    pub stamp: u8,
    pub sub_step: bool,
}
/// Wrapper for unreliable messages.
#[derive(Serialize, Deserialize, Clone)]
pub struct UnreliableMessage {
    pub serialized: Vec<u8>,
    pub typename_net: u8,
}

/// Returns an option containing the desired reliable netcode message.
pub(crate) fn get_reliable_message<T: TypeName + Serialize + for<'a> Deserialize<'a>>(
    typenames: &Res<Typenames>,
    identifier: u16,
    message: &[u8],
) -> Option<T> {
    match typenames.reliable_net_types.get(&T::type_name()) {
        Some(i) => {
            if &identifier == i {
                match bincode::deserialize::<T>(message) {
                    Ok(t) => Some(t),
                    Err(_) => {
                        warn!("Couldnt serialize message!");
                        None
                    }
                }
            } else {
                None
            }
        }
        None => {
            warn!("Couldnt find reliable net type.");
            None
        }
    }
}
use bevy::prelude::Res;
use serde::{Deserialize, Serialize};

/// Typenames systems ordering label.

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum TypenamesSet {
    Generate,
    SendRawEvents,
}
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum MessagingSet {
    DeserializeIncoming,
}
