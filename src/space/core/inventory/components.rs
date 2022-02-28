use bevy_internal::prelude::{Component, Entity};

#[derive(Component)]
pub struct Inventory {
    pub slots: Vec<Slot>,
    pub active_slot: String,
    pub entity_tab_action_option: Option<Entity>,
}

#[derive(Debug)]
pub struct Slot {
    pub slot_type: SlotType,
    pub slot_name: String,
    pub slot_item: Option<Entity>,
    pub slot_attachment: Option<String>,
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            slots: vec![],
            active_slot: "".to_string(),
            entity_tab_action_option: None,
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum SlotType {
    Generic,
    Helmet,
    Jumpsuit,
    Holster,
}

impl Inventory {
    pub fn has_item(&self, entity_id: Entity) -> bool {
        let mut has = false;

        for slot in self.slots.iter() {
            match slot.slot_item {
                Some(item_entity) => {
                    if item_entity == entity_id {
                        has = true;
                        break;
                    }
                }
                None => {}
            }
        }

        has
    }

    pub fn get_active_slot_entity(&self) -> Option<Entity> {
        let mut return_slot_option = None;

        for slot in self.slots.iter() {
            if slot.slot_name == self.active_slot {
                return_slot_option = Some(slot);
                break;
            }
        }

        return_slot_option
            .expect("inventory.rs get_active_entity() couldn't find slot")
            .slot_item
    }

    pub fn get_slot_mut(&mut self, slot_name: &str) -> &mut Slot {
        let mut return_slot_option = None;

        for slot in self.slots.iter_mut() {
            if slot.slot_name == slot_name {
                return_slot_option = Some(slot);
                break;
            }
        }

        return_slot_option.expect("inventory.rs get_slot_mut() couldn't find slot")
    }

    pub fn get_slot(&self, slot_name: &str) -> &Slot {
        let mut return_slot_option = None;

        for slot in self.slots.iter() {
            if slot.slot_name == slot_name {
                return_slot_option = Some(slot);
                break;
            }
        }

        return_slot_option.expect("inventory.rs get_slot() couldn't find slot")
    }
}
