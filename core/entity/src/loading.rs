use bevy::prelude::info;
use bevy::prelude::EventReader;
use networking::client::IncomingReliableServerMessage;

use crate::net::EntityServerMessage;
#[cfg(feature = "client")]
pub(crate) fn load_entities(
    mut client: EventReader<IncomingReliableServerMessage<EntityServerMessage>>,
) {
    for message in client.iter() {
        match &message.message {
            EntityServerMessage::LoadEntity(entity_type, entity_id) => {
                info!("{}", entity_type);
            }
            _ => {}
        }
    }
}
