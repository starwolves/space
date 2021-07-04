use std::collections::HashMap;

use bevy::prelude::{Entity, Transform};

pub struct Pickupable {
    pub in_inventory_of_entity : Option<Entity>,
    pub attachment_transforms : HashMap<String, Transform>,
}
