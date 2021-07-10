use std::collections::HashMap;

use bevy::prelude::{Changed, Query};

use crate::space_core::{components::{entity_updates::EntityUpdates, inventory_item::InventoryItem}, functions::get_entity_update_difference::get_entity_update_difference, structs::network_messages::EntityUpdateData};

pub fn inventory_item_update(
    mut updated_entities: Query<(&InventoryItem, &mut EntityUpdates), Changed<InventoryItem>>,
) {

    
    for (inventory_item_component, mut entity_updates_component) in updated_entities.iter_mut() {

    
        let old_entity_updates = entity_updates_component.updates.clone();


        let mut insert_map = HashMap::new();


        insert_map.insert("worn_is_attached".to_string(), EntityUpdateData::Bool(inventory_item_component.is_attached_when_worn));




        entity_updates_component.updates.insert(".".to_string(), insert_map);





        let difference_updates = get_entity_update_difference(
            old_entity_updates,
            &entity_updates_component.updates
        );

        entity_updates_component.updates_difference = difference_updates;


    }


}
