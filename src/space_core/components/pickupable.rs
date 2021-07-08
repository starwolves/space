use std::collections::HashMap;

use bevy::prelude::{Entity, Transform};

use super::inventory::SlotType;

pub struct Pickupable {
    pub in_inventory_of_entity : Option<Entity>,
    pub attachment_transforms : HashMap<String, Transform>,
    pub drop_transform : Transform,
    pub slot_type : SlotType,
}
