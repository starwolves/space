use bevy::prelude::{Entity, EventReader, EventWriter, Query, Res, warn};

use crate::space_core::{components::{examinable::Examinable, sensable::Sensable}, events::{general::examine_entity::ExamineEntity, net::net_chat_message::NetChatMessage}, resources::handle_to_entity::HandleToEntity, structs::network_messages::ReliableServerMessage};

pub fn examine_entity(
    mut examine_entity_events : EventReader<ExamineEntity>,
    mut net_new_chat_message_event : EventWriter<NetChatMessage>,
    examinable_entities : Query<(&Examinable, &Sensable)>,
    handle_to_entity : Res<HandleToEntity>,
) {

    for examine_event in examine_entity_events.iter() {

        let entity_reference = Entity::new(examine_event.examine_entity_id);

        let entity_components_option = examinable_entities.get(entity_reference);

        match entity_components_option {
            Ok((examinable_component, sensable_component)) => {
                
                
                let entity = handle_to_entity.map.get(&examine_event.handle)
                .expect("examine_entity.rs could not find the entity belonging to examining handle.");

                let text;

                if sensable_component.sensed_by.contains(entity) {

                    text = examinable_component.text.clone();

                } else {

                    text = "You cannot see what is there.".to_owned();

                }

                net_new_chat_message_event.send(NetChatMessage {
                    handle: examine_event.handle,
                    message: ReliableServerMessage::ChatMessage(text),
                });

            },
            Err(_rr) => {
                warn!("Couldn't find examinable entity.");
            },
        }

    }

}
