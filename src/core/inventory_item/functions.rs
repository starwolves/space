use bevy_ecs::{entity::Entity, system::Commands};

use crate::core::rigid_body::components::RigidBodyLinkTransform;

use super::components::InventoryItem;

pub struct InventoryBuilderData {
    pub inventory_item: InventoryItem,
    pub holder_entity_option: Option<Entity>,
}

pub fn inventory_item_builder(commands: &mut Commands, entity: Entity, data: InventoryBuilderData) {
    let mut builder = commands.entity(entity);
    builder.insert_bundle((data.inventory_item,));
    match data.holder_entity_option {
        Some(holder_entity) => {
            builder.insert(RigidBodyLinkTransform {
                follow_entity: holder_entity,
                ..Default::default()
            });
        }
        None => {}
    }
}
