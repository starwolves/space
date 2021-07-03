use bevy::prelude::Entity;

pub struct Pickupable {
    pub in_inventory_of_entity : Option<Entity>,
}
