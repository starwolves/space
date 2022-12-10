use std::collections::HashMap;

use bevy::prelude::Resource;
use networking::server::IncomingReliableClientMessage;
use networking::server::NetworkingClientMessage;

use bevy::prelude::{EventReader, EventWriter};

use crate::connection::SendServerConfiguration;

/// Player accounts stored with handles.
#[derive(Default, Resource)]
#[cfg(feature = "server")]
pub struct Accounts {
    pub list: HashMap<u64, String>,
}
use bevy::prelude::ResMut;

/// Client account verification.
#[cfg(feature = "server")]
pub(crate) fn account_verification(
    mut incoming: EventReader<IncomingReliableClientMessage<NetworkingClientMessage>>,
    mut configure: EventWriter<SendServerConfiguration>,
    mut accounts: ResMut<Accounts>,
) {
    for message in incoming.iter() {
        match &message.message {
            NetworkingClientMessage::Account(account_name) => {
                accounts.list.insert(message.handle, account_name.clone());
                configure.send(SendServerConfiguration {
                    handle: message.handle,
                })
            }
            _ => {}
        }
    }
}
