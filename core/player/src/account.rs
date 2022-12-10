use std::collections::HashMap;

use bevy::prelude::Res;
use bevy::prelude::Resource;
use networking::server::IncomingReliableClientMessage;
use networking::server::NetworkingClientMessage;

use bevy::prelude::{EventReader, EventWriter};

/// Player accounts stored with handles.
#[derive(Default, Resource)]
#[cfg(feature = "server")]
pub struct Accounts {
    pub list: HashMap<u64, String>,
}
use crate::connections::SendServerConfiguration;
use crate::names::UsedNames;
use bevy::prelude::ResMut;
use networking::server::{NetworkingServerMessage, OutgoingReliableServerMessage};

/// Client account verification.
#[cfg(feature = "server")]
pub(crate) fn account_verification(
    mut incoming: EventReader<IncomingReliableClientMessage<NetworkingClientMessage>>,
    mut outgoing: EventWriter<OutgoingReliableServerMessage<NetworkingServerMessage>>,
    mut configure: EventWriter<SendServerConfiguration>,
    mut accounts: ResMut<Accounts>,
    used_names: Res<UsedNames>,
) {
    for message in incoming.iter() {
        match &message.message {
            NetworkingClientMessage::Account(account_name) => {
                let mut user_name = account_name.clone();
                if user_name.len() > 16 {
                    user_name = user_name[..16].to_string();
                }

                if used_names.account_name.contains_key(&user_name) {
                    //Already exists.
                    continue;
                }
                accounts.list.insert(message.handle, user_name);

                outgoing.send(OutgoingReliableServerMessage {
                    handle: message.handle,
                    message: NetworkingServerMessage::Awoo,
                });

                configure.send(SendServerConfiguration {
                    handle: message.handle,
                })
            }
            _ => {}
        }
    }
}
