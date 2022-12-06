use std::collections::HashMap;

use bevy::prelude::{App, EventReader, SystemLabel};
use bevy::prelude::{ResMut, Resource};
use typename::TypeName;

/// Resource containing typenames and smaller 16-bit netcode representations. Needed to identify Rust types sent over the net.
#[cfg(any(feature = "server", feature = "client"))]
#[derive(Resource, Default)]
pub struct Typenames {
    pub reliable_incremental_id: u16,
    pub unreliable_incremental_id: u8,
    pub reliable_types: Vec<String>,
    pub unreliable_types: Vec<String>,
    pub reliable_net_types: HashMap<String, u16>,
    pub unreliable_net_types: HashMap<String, u8>,
}

use bevy::prelude::warn;

/// Generic startup system that registers reliable netcode message types. All reliable netcode types sent over the net must be registered with this system.
#[cfg(any(feature = "server", feature = "client"))]
pub fn reliable_message<T: TypeName>(mut typenames: ResMut<Typenames>) {
    typenames.reliable_types.push(T::type_name());
}
/// Generic startup system that registers unreliable netcode message types. All unreliable netcode types sent over the net must be registered with this system.
#[cfg(any(feature = "server", feature = "client"))]
pub fn unreliable_message<T: TypeName>(mut typenames: ResMut<Typenames>) {
    typenames.unreliable_types.push(T::type_name());
}
use bevy::prelude::info;

/// Order and generate typenames.
#[cfg(any(feature = "server", feature = "client"))]
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
        "Generated {} typenames.",
        typenames.reliable_net_types.len() + typenames.unreliable_net_types.len()
    );
}
use bevy::app::CoreStage::PostUpdate;
use bevy::prelude::IntoSystemDescriptor;

pub enum MessageSender {
    Client,
    Server,
    Both,
}

use iyes_loopless::prelude::IntoConditionalSystem;
use std::env;

use crate::client::connected;
#[cfg(any(feature = "server", feature = "client"))]
pub fn init_reliable_message<T: TypeName + Send + Sync + Serialize + 'static>(
    app: &mut App,
    sender: MessageSender,
) {
    app.add_startup_system(reliable_message::<T>.label(TypenamesLabel::Generate));

    let mut build_client = false;
    let mut build_server = false;

    match sender {
        MessageSender::Client => {
            build_client = true;
        }
        MessageSender::Server => {
            build_server = true;
        }
        MessageSender::Both => {
            build_client = true;
            build_server = true;
        }
    }

    if build_server && env::var("CARGO_MANIFEST_DIR").unwrap().ends_with("server") {
        app.add_event::<OutgoingReliableServerMessage<T>>()
            .add_system_to_stage(PostUpdate, send_outgoing_reliable_server_messages::<T>);
    }
    if build_client && env::var("CARGO_MANIFEST_DIR").unwrap().ends_with("client") {
        app.add_event::<OutgoingReliableClientMessage<T>>()
            .add_system_to_stage(
                PostUpdate,
                send_outgoing_reliable_client_messages::<T>.run_if(connected),
            );
    }
}
pub fn init_unreliable_message<T: TypeName + Send + Sync + Serialize + 'static>(
    app: &mut App,
    sender: MessageSender,
) {
    app.add_startup_system(unreliable_message::<T>.label(TypenamesLabel::Generate));
    let mut build_client = false;
    let mut build_server = false;

    match sender {
        MessageSender::Client => {
            build_client = true;
        }
        MessageSender::Server => {
            build_server = true;
        }
        MessageSender::Both => {
            build_client = true;
            build_server = true;
        }
    }
    if build_server && env::var("CARGO_MANIFEST_DIR").unwrap().ends_with("server") {
        app.add_event::<OutgoingUnreliableServerMessage<T>>()
            .add_system_to_stage(PostUpdate, send_outgoing_unreliable_server_messages::<T>);
    }
    if build_client && env::var("CARGO_MANIFEST_DIR").unwrap().ends_with("client") {
        app.add_event::<OutgoingUnreliableClientMessage<T>>()
            .add_system_to_stage(
                PostUpdate,
                send_outgoing_unreliable_client_messages::<T>.run_if(connected),
            );
    }
}

/// Wrapper for reliable messages.
#[derive(Serialize, Deserialize)]
#[cfg(any(feature = "server", feature = "client"))]
pub struct ReliableMessage {
    pub message: Vec<u8>,
    pub typename_net: u16,
}
/// Wrapper for unreliable messages.
#[derive(Serialize, Deserialize)]
#[cfg(any(feature = "server", feature = "client"))]
pub struct UnreliableMessage {
    pub message: Vec<u8>,
    pub typename_net: u8,
}

/// Event to send reliable messages from server to client.
#[cfg(any(feature = "server"))]
pub struct OutgoingReliableServerMessage<T: TypeName + Send + Sync + 'static> {
    pub handle: u64,
    pub message: T,
}
/// Event to when received reliable message from server.
#[cfg(any(feature = "client"))]
pub struct IncomingReliableServerMessage {
    pub message: ReliableMessage,
}
#[cfg(any(feature = "client"))]
pub fn identify_reliable_server_input<T: TypeName>(
    typenames: &Res<Typenames>,
    identifier: u16,
) -> bool {
    match typenames.reliable_net_types.get(&T::type_name()) {
        Some(i) => &identifier == i,
        None => {
            warn!("Couldnt find reliable net type.");
            false
        }
    }
}

#[cfg(any(feature = "client"))]
pub fn identify_unreliable_server_input<T: TypeName>(
    typenames: &Res<Typenames>,
    identifier: u8,
) -> bool {
    match typenames.unreliable_net_types.get(&T::type_name()) {
        Some(i) => &identifier == i,
        None => {
            warn!("Couldnt find unreliable net type.");
            false
        }
    }
}

/// Event to when received reliable message from server.
#[cfg(any(feature = "client"))]
pub struct IncomingUnreliableServerMessage {
    pub message: UnreliableMessage,
}
/// Event to send reliable messages from client to server.
#[cfg(any(feature = "client"))]
pub struct OutgoingReliableClientMessage<T: TypeName + Send + Sync + 'static> {
    pub message: T,
}

/// Event to send unreliable messages from client to server.
#[cfg(any(feature = "client"))]
pub struct OutgoingUnreliableClientMessage<T: TypeName + Send + Sync + 'static> {
    pub message: T,
}
/// Event to send unreliable messages from server to client.
#[cfg(any(feature = "server"))]
pub struct OutgoingUnreliableServerMessage<T: TypeName + Send + Sync + 'static> {
    pub handle: u64,
    pub message: T,
}
use crate::plugin::RENET_RELIABLE_CHANNEL_ID;
use bevy::prelude::EventWriter;
use bevy::prelude::Res;
use bevy_renet::renet::RenetServer;
use serde::{Deserialize, Serialize};

/// Deserializes incoming server messages and writes to event.
#[cfg(any(feature = "client"))]
pub(crate) fn receive_incoming_reliable_server_messages(
    mut events: EventWriter<IncomingReliableServerMessage>,
    mut client: ResMut<RenetClient>,
) {
    while let Some(message) = client.receive_message(RENET_RELIABLE_CHANNEL_ID) {
        match bincode::deserialize::<ReliableMessage>(&message) {
            Ok(msg) => {
                events.send(IncomingReliableServerMessage { message: msg });
            }
            Err(_) => {
                warn!("Received an invalid message.");
            }
        }
    }
}

/// Dezerializes incoming server messages and writes to event.
#[cfg(any(feature = "client"))]
pub(crate) fn receive_incoming_unreliable_server_messages(
    mut events: EventWriter<IncomingUnreliableServerMessage>,
    mut client: ResMut<RenetClient>,
) {
    while let Some(message) = client.receive_message(RENET_UNRELIABLE_CHANNEL_ID) {
        match bincode::deserialize::<UnreliableMessage>(&message) {
            Ok(msg) => {
                events.send(IncomingUnreliableServerMessage { message: msg });
            }
            Err(_) => {
                warn!("Received an invalid message.");
            }
        }
    }
}

/// Serializes and sends the outgoing reliable server messages.
#[cfg(any(feature = "server"))]
pub(crate) fn send_outgoing_reliable_server_messages<T: TypeName + Send + Sync + Serialize>(
    mut events: EventReader<OutgoingReliableServerMessage<T>>,
    mut server: ResMut<RenetServer>,
    typenames: Res<Typenames>,
) {
    for message in events.iter() {
        let net;
        match typenames
            .reliable_net_types
            .get(&message.message.type_name_of())
        {
            Some(n) => {
                net = n;
            }
            None => {
                warn!(
                    "Couldnt find server reliable type {}",
                    message.message.type_name_of()
                );
                continue;
            }
        }
        let bin;
        match bincode::serialize(&message.message) {
            Ok(b) => {
                bin = b;
            }
            Err(_) => {
                warn!("Couldnt serialize reliable message");
                continue;
            }
        }

        match bincode::serialize(&ReliableMessage {
            message: bin,
            typename_net: *net,
        }) {
            Ok(bits) => {
                server.send_message(message.handle, RENET_RELIABLE_CHANNEL_ID, bits);
            }
            Err(_) => {
                warn!("Failed to serialize reliable message.");
                continue;
            }
        }
    }
}
use bevy_renet::renet::RenetClient;

/// Serializes and sends the outgoing reliable client messages.
#[cfg(any(feature = "client"))]
pub(crate) fn send_outgoing_reliable_client_messages<T: TypeName + Send + Sync + Serialize>(
    mut events: EventReader<OutgoingReliableClientMessage<T>>,
    mut client: ResMut<RenetClient>,
    typenames: Res<Typenames>,
) {
    for message in events.iter() {
        let net;
        match typenames
            .reliable_net_types
            .get(&message.message.type_name_of())
        {
            Some(n) => {
                net = n;
            }
            None => {
                warn!("Couldnt find client reliable type");
                continue;
            }
        }
        let bin;
        match bincode::serialize(&message.message) {
            Ok(b) => {
                bin = b;
            }
            Err(_) => {
                warn!("Couldnt serialize reliable message");
                continue;
            }
        }

        match bincode::serialize(&ReliableMessage {
            message: bin,
            typename_net: *net,
        }) {
            Ok(bits) => {
                client.send_message(RENET_RELIABLE_CHANNEL_ID, bits);
            }
            Err(_) => {
                warn!("Failed to serialize reliable message.");
                continue;
            }
        }
    }
}
use crate::plugin::RENET_UNRELIABLE_CHANNEL_ID;

/// Serializes and sends the outgoing unreliable server messages.
#[cfg(any(feature = "server"))]
pub(crate) fn send_outgoing_unreliable_server_messages<T: TypeName + Send + Sync + Serialize>(
    mut events: EventReader<OutgoingUnreliableServerMessage<T>>,
    mut server: ResMut<RenetServer>,
    typenames: Res<Typenames>,
) {
    for message in events.iter() {
        let net;
        match typenames
            .unreliable_net_types
            .get(&message.message.type_name_of())
        {
            Some(n) => {
                net = n;
            }
            None => {
                warn!("Couldnt find unreliable type");
                continue;
            }
        }
        let bin;
        match bincode::serialize(&message.message) {
            Ok(b) => {
                bin = b;
            }
            Err(_) => {
                warn!("Couldnt serialize unreliable message");
                continue;
            }
        }

        match bincode::serialize(&UnreliableMessage {
            message: bin,
            typename_net: *net,
        }) {
            Ok(bits) => {
                server.send_message(message.handle, RENET_UNRELIABLE_CHANNEL_ID, bits);
            }
            Err(_) => {
                warn!("Failed to serialize unreliable message.");
                continue;
            }
        }
    }
}

/// Serializes and sends the outgoing unreliable client messages.
#[cfg(any(feature = "client"))]
pub(crate) fn send_outgoing_unreliable_client_messages<T: TypeName + Send + Sync + Serialize>(
    mut events: EventReader<OutgoingUnreliableClientMessage<T>>,
    mut client: ResMut<RenetClient>,
    typenames: Res<Typenames>,
) {
    for message in events.iter() {
        let net;
        match typenames
            .unreliable_net_types
            .get(&message.message.type_name_of())
        {
            Some(n) => {
                net = n;
            }
            None => {
                warn!("Couldnt find unreliable type");
                continue;
            }
        }
        let bin;
        match bincode::serialize(&message.message) {
            Ok(b) => {
                bin = b;
            }
            Err(_) => {
                warn!("Couldnt serialize unreliable message");
                continue;
            }
        }

        match bincode::serialize(&UnreliableMessage {
            message: bin,
            typename_net: *net,
        }) {
            Ok(bits) => {
                client.send_message(RENET_UNRELIABLE_CHANNEL_ID, bits);
            }
            Err(_) => {
                warn!("Failed to serialize unreliable message.");
                continue;
            }
        }
    }
}
/// Typenames systems ordering label.
#[cfg(any(feature = "server"))]
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum TypenamesLabel {
    Generate,
}
