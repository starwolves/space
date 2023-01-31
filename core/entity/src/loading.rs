use bevy::prelude::info;
use bevy::prelude::EventReader;
use networking::client::IncomingReliableServerMessage;

use bevy::prelude::Res;

use crate::entity_types::EntityTypes;
use crate::net::EntityServerMessage;

pub(crate) fn load_entities(
    mut client: EventReader<IncomingReliableServerMessage<EntityServerMessage>>,
    types: Res<EntityTypes>,
) {
    for message in client.iter() {
        match &message.message {
            EntityServerMessage::LoadEntity(entity_type, _entity) => {
                let index = types
                    .netcode_types
                    .values()
                    .position(|r| r == entity_type)
                    .unwrap();
                let keys: Vec<&String> = types.netcode_types.keys().collect();
                info!("{}", keys.get(index).unwrap());
            }
            _ => {}
        }
    }
}
