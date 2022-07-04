use bevy::prelude::{Commands, Entity, EventReader};

use crate::core::{
    entity::spawn::{SpawnData, SpawnEvent},
    rigid_body::rigidbody_link_transform::RigidBodyLinkTransform,
};

use super::item::InventoryItem;

pub struct InventoryItemBundle {
    pub inventory_item: InventoryItem,
}

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
pub trait InventoryItemSummonable {
    fn get_bundle(&self, spawn_data: &SpawnData) -> InventoryItemBundle;
}

pub fn summon_inventory_item<T: InventoryItemSummonable + Send + Sync + 'static>(
    mut spawn_events: EventReader<SpawnEvent<T>>,
    mut commands: Commands,
) {
    for spawn_event in spawn_events.iter() {
        let inventory_item_bundle = spawn_event.summoner.get_bundle(&spawn_event.spawn_data);

        inventory_item_builder(
            &mut commands,
            spawn_event.spawn_data.entity,
            InventoryBuilderData {
                inventory_item: inventory_item_bundle.inventory_item,
                holder_entity_option: spawn_event.spawn_data.holder_entity_option,
            },
        );
    }
}
