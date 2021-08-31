use std::collections::HashMap;

use bevy::prelude::{Changed, Entity, Query, QuerySet, warn};

use crate::space_core::{bundles::human_male_pawn::generate_human_examine_text, components::{entity_data::EntityData, entity_updates::EntityUpdates, examinable::Examinable, inventory::Inventory, inventory_item::InventoryItem, standard_character::StandardCharacter}, functions::{entity_updates::get_entity_update_difference::get_entity_update_difference}, resources::network_messages::EntityUpdateData};

pub fn inventory_update(
    mut updated_entities: Query
    <(
        Entity, 
        &Inventory, 
        &mut EntityUpdates, 
        Option<&StandardCharacter>
    ), 
        Changed<Inventory>
    >,
    pickupables : Query<(
        &InventoryItem,
        &EntityData
    )>,
    mut query_set: QuerySet<(
        Query<&mut Examinable>,
        Query<&Examinable>,
    )>,
) {
    
    for (updated_entity, 
        inventory_component, 
        mut entity_updates_component, 
        standard_character_option
    ) in updated_entities.iter_mut() {
        
        let old_entity_updates = entity_updates_component.updates.clone();

        let mut new_examine_text_option = None;

        match standard_character_option {
            Some(standard_character_component) => {
                new_examine_text_option = Some(generate_human_examine_text(
                    &standard_character_component.character_name,
                    Some(inventory_component),
                    Some(query_set.q1_mut())
                ));
            },
            None => {},
        }

        

        match new_examine_text_option {
            Some(new_examine_text) => {

                let mut examinable_entity_component = query_set.q0_mut().get_mut(updated_entity).unwrap();

                examinable_entity_component.description = new_examine_text;

            },
            None => {},
        }

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

                    
                    let mut update_map = HashMap::new();

                    let attachment_transform_option = pickupable_components.0.attachment_transforms.get(&slot.slot_name);

                    match attachment_transform_option {
                        Some(attachment_transform) => {
                            
                            update_map.insert("attachedItem".to_string(), EntityUpdateData::AttachedItem(item.to_bits(), attachment_transform.translation, attachment_transform.rotation, attachment_transform.scale));
    
                            update_map.insert("wornItems".to_string(), EntityUpdateData::WornItem(slot.slot_name.clone(), item.to_bits(), pickupable_components.1.entity_type.clone(), attachment_transform.translation, attachment_transform.rotation, attachment_transform.scale));

                        },
                        None => {

                            if pickupable_components.0.is_attached_when_worn == true {

                                warn!("inventory_update.rs couldn't find pickupable attachment transform for used slot name.");


                            } else {
                                                                
                                update_map.insert("wornItems".to_string(), EntityUpdateData::WornItemNotAttached(slot.slot_name.clone(), item.to_bits(), pickupable_components.1.entity_type.clone()));
        
                            }

                        },
                    }
                    
                    entity_updates_component.updates.insert(attachment_slot.to_string(), update_map);

                },
                None => {
                    entity_updates_component.updates.remove(attachment_slot);
                },
            }

        }

        let difference_updates = get_entity_update_difference(
            old_entity_updates,
            &entity_updates_component.updates
        );

        entity_updates_component.updates_difference.push(difference_updates);

    }

}
