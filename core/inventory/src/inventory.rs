use bevy::prelude::{Component, Entity};

#[derive(PartialEq, Copy, Clone, Debug)]
#[cfg(feature = "server")]
pub enum SlotType {
    Generic,
    Helmet,
    Jumpsuit,
    Holster,
}

/// An inventory slot, an inventory can contain many of these.
#[cfg(feature = "server")]
pub struct Slot {
    pub slot_type: SlotType,
    pub slot_name: String,
    pub slot_item: Option<Entity>,
    /// The id of the attachment of this slot.
    pub slot_attachment: Option<String>,
}
/// The inventory component.
#[derive(Component)]
#[cfg(feature = "server")]
pub struct Inventory {
    pub slots: Vec<Slot>,
    pub active_slot: String,
}

#[cfg(feature = "server")]
impl Default for Inventory {
    fn default() -> Self {
        Self {
            slots: vec![],
            active_slot: "".to_string(),
        }
    }
}

#[cfg(feature = "server")]
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
