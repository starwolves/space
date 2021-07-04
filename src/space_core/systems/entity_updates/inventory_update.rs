use std::collections::HashMap;

use bevy::prelude::{Changed, Query};

use crate::space_core::{components::{entity_updates::EntityUpdates, inventory::Inventory, pickupable::Pickupable}, structs::network_messages::EntityUpdateData};

pub fn inventory_update(
    mut updated_entities: Query<(&Inventory, &mut EntityUpdates), Changed<Inventory>>,
    pickupables : Query<&Pickupable>,
) {
    
    for (inventory_component, mut entity_updates_component) in updated_entities.iter_mut() {
                
        for slot in &inventory_component.slots {

            let attachment_slot;

            match &slot.slot_attachment {
                Some(attachment) => {
                    attachment_slot = attachment;
                },
                None => {
                    continue;
                },
            }

            match slot.slot_item {
                Some(item) => {
                    
                    let pickupable_components = pickupables.get(item)
                    .expect("inventory_update.rs couldn't find pickupable entity in query that is in inventory slot.");

                    let attachment_transform = pickupable_components.attachment_transforms.get(&slot.slot_name)
                    .expect("inventory_update.rs couldn't pickupable attachment transform for used slot name.");
                    
                    let mut update_map = HashMap::new();

                    update_map.insert("attachedItem".to_string(), EntityUpdateData::AttachedItem(item.id(), attachment_transform.translation, attachment_transform.rotation, attachment_transform.scale));

                    entity_updates_component.updates.insert(attachment_slot.to_string(), update_map);

                },
                None => {
                    entity_updates_component.updates.remove(attachment_slot);
                },
            }

        }

    }

}
