use std::collections::HashMap;

use api::{
    entity_updates::{get_entity_update_difference, EntityData, EntityUpdateData, EntityUpdates},
    inventory::Inventory,
};
use bevy::prelude::{warn, Changed, Query};
use inventory_item::item::InventoryItem;

/// Inventory item update.
pub(crate) fn inventory_update(
    mut updated_entities: Query<(&Inventory, &mut EntityUpdates), Changed<Inventory>>,
    pickupables: Query<(&InventoryItem, &EntityData)>,
) {
    for (inventory_component, mut entity_updates_component) in updated_entities.iter_mut() {
        let old_entity_updates = entity_updates_component.updates.clone();

        for slot in &inventory_component.slots {
            let attachment_slot;

            match &slot.slot_attachment {
                Some(attachment) => {
                    attachment_slot = attachment;
                }
                None => {
                    continue;
                }
            }

            match slot.slot_item {
                Some(item) => {
                    let pickupable_components = pickupables.get(item)
                    .expect("inventory_update.rs couldn't find pickupable entity in query that is in inventory slot.");

                    let mut update_map = HashMap::new();

                    let attachment_transform_option = pickupable_components
                        .0
                        .attachment_transforms
                        .get(&slot.slot_name);

                    match attachment_transform_option {
                        Some(attachment_transform) => {
                            update_map.insert(
                                "attachedItem".to_string(),
                                EntityUpdateData::AttachedItem(
                                    item.to_bits(),
                                    attachment_transform.translation,
                                    attachment_transform.rotation,
                                    attachment_transform.scale,
                                ),
                            );

                            update_map.insert(
                                "wornItems".to_string(),
                                EntityUpdateData::WornItem(
                                    slot.slot_name.clone(),
                                    item.to_bits(),
                                    pickupable_components.1.entity_name.clone(),
                                    attachment_transform.translation,
                                    attachment_transform.rotation,
                                    attachment_transform.scale,
                                ),
                            );
                        }
                        None => {
                            if pickupable_components.0.is_attached_when_worn == true {
                                warn!("inventory_update.rs couldn't find pickupable attachment transform for used slot name.");
                            } else {
                                update_map.insert(
                                    "wornItems".to_string(),
                                    EntityUpdateData::WornItemNotAttached(
                                        slot.slot_name.clone(),
                                        item.to_bits(),
                                        pickupable_components.1.entity_name.clone(),
                                    ),
                                );
                            }
                        }
                    }

                    entity_updates_component
                        .updates
                        .insert(attachment_slot.to_string(), update_map);
                }
                None => {
                    entity_updates_component.updates.remove(attachment_slot);
                }
            }
        }

        let difference_updates =
            get_entity_update_difference(old_entity_updates, &entity_updates_component.updates);

        entity_updates_component
            .updates_difference
            .push(difference_updates);
    }
}
