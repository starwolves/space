use std::collections::HashMap;

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
use bevy::prelude::warn;
use bevy::prelude::ResMut;
use networking::server::{NetworkingServerMessage, OutgoingReliableServerMessage};

/// Client account verification.
#[cfg(feature = "server")]
pub(crate) fn account_verification(
    mut incoming: EventReader<IncomingReliableClientMessage<NetworkingClientMessage>>,
    mut outgoing: EventWriter<OutgoingReliableServerMessage<NetworkingServerMessage>>,
    mut configure: EventWriter<SendServerConfiguration>,
    mut accounts: ResMut<Accounts>,
    mut used_names: ResMut<UsedNames>,
) {
    for message in incoming.iter() {
        match &message.message {
            NetworkingClientMessage::Account(account_name) => {
                let mut user_name = account_name.clone();
                if user_name.len() > 16 {
                    user_name = user_name[..16].to_string();
                }

                if used_names.used_account_names.contains(&user_name) {
                    // Account name already exists.

                    let mut default_name = "Wolf".to_string() + &used_names.player_i.to_string();
                    used_names.player_i += 1;

                    while used_names.used_account_names.contains(&default_name) {
                        used_names.player_i += 1;
                        default_name = "Wolf".to_string() + &used_names.player_i.to_string();
                    }

                    warn!(
                        "Account name {} by [{}] already exists. Assigned account name {}.",
                        user_name, message.handle, default_name
                    );

                    user_name = default_name;
                }
                used_names.used_account_names.push(user_name.clone());
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
