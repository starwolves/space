use bevy::prelude::{Changed, Entity, EventWriter, Query, ResMut};

use crate::space_core::{components::{entity_updates::EntityUpdates, sensable::Sensable}, events::net::net_send_entity_updates::NetSendEntityUpdates, resources::handle_to_entity::HandleToEntity, structs::network_messages::ReliableServerMessage};

pub fn send_entity_updates(
    mut updated_entity_updates: Query<(Entity, &Sensable, &mut EntityUpdates), Changed<EntityUpdates>>,
    mut net_send_entity_updates: EventWriter<NetSendEntityUpdates>,
    handle_to_entity: ResMut<HandleToEntity>
) {

    for (visible_entity, visible_component, entity_updates_component) in updated_entity_updates.iter_mut() {

        let visible_entity_id = visible_entity.id();

        for sensed_by_entity in visible_component.sensed_by.iter() {

            net_send_entity_updates.send(NetSendEntityUpdates {
                handle: *handle_to_entity.inv_map.get(&sensed_by_entity.id())
                .expect("send_entity_updates.rs could not find entity id in handle_to_entity.inv_map"),
                message: ReliableServerMessage::EntityUpdate(visible_entity_id, entity_updates_component.updates.clone())
            });

        }

    }

}
