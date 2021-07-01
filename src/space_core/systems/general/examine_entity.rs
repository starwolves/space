use bevy::prelude::{Entity, EventReader, EventWriter, Query, warn};

use crate::space_core::{components::examinable::Examinable, events::{general::examine_entity::ExamineEntity, net::net_chat_message::NetChatMessage}, structs::network_messages::ReliableServerMessage};

pub fn examine_entity(
    mut examine_entity_events : EventReader<ExamineEntity>,
    mut net_new_chat_message_event : EventWriter<NetChatMessage>,
    examinable_entities : Query<&Examinable>,
) {

    for examine_event in examine_entity_events.iter() {

        let entity_reference = Entity::new(examine_event.examine_entity_id);

        let entity_components_option = examinable_entities.get(entity_reference);

        match entity_components_option {
            Ok(examinable_component) => {
                
                net_new_chat_message_event.send(NetChatMessage {
                    handle: examine_event.handle,
                    message: ReliableServerMessage::ChatMessage(examinable_component.text.clone()),
                });

            },
            Err(_rr) => {
                warn!("Couldn't find examinable entity.");
            },
        }

    }

}
