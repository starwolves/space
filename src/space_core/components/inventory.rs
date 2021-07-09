use bevy::prelude::Entity;

pub struct Inventory {
    pub slots : Vec<Slot>,
    pub pickup_slot : String,
}

pub struct Slot {
    pub slot_type : SlotType,
    pub slot_name : String,
    pub slot_item : Option<Entity>,
    pub slot_attachment : Option<String>,
}


#[derive(Copy, Clone)]
pub enum SlotType {
    Generic,
    Helmet,
}

impl Inventory {

    pub fn get_slot_mut(&mut self, slot_name : &str) -> &mut Slot {

        let mut return_slot_option = None;

        for slot in self.slots.iter_mut() {

            if slot.slot_name == slot_name {
                return_slot_option = Some(slot);
                break;
            }

        }

        return_slot_option.expect("inventory.rs get_slot() couldn't find slot")

    }

}
