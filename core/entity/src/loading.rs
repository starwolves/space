use bevy::prelude::info;
use bevy::prelude::EventReader;
use networking::client::IncomingReliableServerMessage;

use bevy::prelude::Res;

use crate::entity_types::EntityTypes;
use crate::net::EntityServerMessage;
#[cfg(feature = "client")]
pub(crate) fn load_entities(
    mut client: EventReader<IncomingReliableServerMessage<EntityServerMessage>>,
    types: Res<EntityTypes>,
) {
    for message in client.iter() {
        match &message.message {
            EntityServerMessage::LoadEntity(entity_type, entity_id) => {
                let index = types.types.values().position(|r| r == entity_type).unwrap();
                let keys: Vec<&String> = types.types.keys().collect();
                info!("{}", keys.get(index).unwrap());
            }
            _ => {}
        }
    }
}
