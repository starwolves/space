use api::inventory::Inventory;
use bevy::prelude::{warn, Changed, Query};
use inventory_item::item::InventoryItem;
use pawn::pawn::Pawn;

pub fn inventory_tab_data(
    mut changed_inventories: Query<(&mut Inventory, &mut Pawn), Changed<Inventory>>,
    inventory_items: Query<&InventoryItem>,
) {
    for (mut inventory_component, mut pawn_component) in changed_inventories.iter_mut() {
        let active_entity_option = inventory_component.get_active_slot_entity();

        match &inventory_component.entity_tab_action_option {
            Some(entity_tab_entity) => match active_entity_option {
                Some(entity) => {
                    pawn_component.tab_actions_remove_entity(Some(*entity_tab_entity));

                    match inventory_items.get(entity) {
                        Ok(inventory_item_component) => {
                            for item_tab_action in
                                inventory_item_component.active_slot_tab_actions.iter()
                            {
                                pawn_component.tab_actions_add(
                                    &item_tab_action.id,
                                    Some(*entity_tab_entity),
                                    item_tab_action.clone(),
                                );
                            }
                        }
                        Err(_rr) => {
                            warn!("Couldn't find inventory_item_component");
                        }
                    }

                    inventory_component.entity_tab_action_option = Some(*entity_tab_entity);
                }
                None => {
                    pawn_component.tab_actions_remove_entity(Some(*entity_tab_entity));
                }
            },
            None => match active_entity_option {
                Some(entity) => {
                    match inventory_items.get(entity) {
                        Ok(inventory_item_component) => {
                            for item_tab_action in
                                inventory_item_component.active_slot_tab_actions.iter()
                            {
                                pawn_component.tab_actions_add(
                                    &item_tab_action.id,
                                    Some(entity),
                                    item_tab_action.clone(),
                                );
                            }
                        }
                        Err(_rr) => {
                            warn!("Couldn't find inventory_item_component (0)");
                        }
                    }

                    inventory_component.entity_tab_action_option = Some(entity);
                }
                None => {}
            },
        }
    }
}
