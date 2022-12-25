use bevy::prelude::{Commands, Entity, EventReader};
use entity::spawn::{EntityBuildData, SpawnEntity};

use crate::combat::{MeleeCombat, ProjectileCombat};

use super::item::InventoryItem;

/// Inventory item bundle.
#[cfg(any(feature = "server", feature = "client"))]
pub struct InventoryItemBundle {
    pub inventory_item: InventoryItem,
    pub melee_combat: MeleeCombat,
    pub projectile_combat_option: Option<ProjectileCombat>,
}

/// Inventory item builder data.
#[cfg(any(feature = "server", feature = "client"))]
pub struct InventoryBuilderData {
    pub inventory_item: InventoryItem,
    pub holder_entity_option: Option<Entity>,
    pub melee_combat: MeleeCombat,
    pub projectile_option: Option<ProjectileCombat>,
}
use physics::physics::RigidBodyLinkTransform;

/// Build inventory item at building stage.
#[cfg(any(feature = "server", feature = "client"))]
pub(crate) fn inventory_item_builder(
    commands: &mut Commands,
    entity: Entity,
    data: InventoryBuilderData,
) {
    let mut builder = commands.entity(entity);
    builder.insert((data.inventory_item, data.melee_combat));
    match data.holder_entity_option {
        Some(holder_entity) => {
            builder.insert(RigidBodyLinkTransform {
                follow_entity: holder_entity,
                ..Default::default()
            });
            match data.projectile_option {
                Some(c) => {
                    builder.insert(c);
                }
                None => {}
            }
        }
        None => {}
    }
}
/// Inventory item buildable.
#[cfg(any(feature = "server", feature = "client"))]
pub trait InventoryItemBuilder: Send + Sync {
    fn get_bundle(&self, spawn_data: &EntityBuildData) -> InventoryItemBundle;
}

/// Inventory item spawn handler.
#[cfg(any(feature = "server", feature = "client"))]
pub fn build_inventory_items<T: InventoryItemBuilder + 'static>(
    mut spawn_events: EventReader<SpawnEntity<T>>,
    mut commands: Commands,
) {
    for spawn_event in spawn_events.iter() {
        let inventory_item_bundle = spawn_event.builder.get_bundle(&spawn_event.spawn_data);

        inventory_item_builder(
            &mut commands,
            spawn_event.spawn_data.entity,
            InventoryBuilderData {
                inventory_item: inventory_item_bundle.inventory_item,
                holder_entity_option: spawn_event.spawn_data.holder_entity_option,
                melee_combat: inventory_item_bundle.melee_combat,
                projectile_option: inventory_item_bundle.projectile_combat_option,
            },
        );
    }
}
/*
/// Function to spawn an entity that is held in someone's hands.
#[cfg(any(feature = "server", feature = "client"))]
pub fn spawn_held_entity<T: EntityType + Send + Sync + 'static>(
    entity_type: T,
    commands: &mut Commands,
    holder_entity: Entity,
    showcase_handle_option: Option<ShowcaseData>,
    entity_data: &ResMut<EntityDataResource>,
    default_spawner: &mut EventWriter<DefaultSpawnEvent<T>>,
) -> Option<Entity> {
    let return_entity;

    match entity_data.name_to_id.get(&entity_type.to_string()) {
        Some(_id) => {
            return_entity = Some(commands.spawn(()).id());

            default_spawner.send(DefaultSpawnEvent {
                spawn_data: EntityBuildData {
                    entity_transform: Transform::IDENTITY,
                    correct_transform: false,
                    holder_entity_option: Some(holder_entity),
                    default_map_spawn: false,
                    raw_entity_option: None,
                    showcase_data_option: showcase_handle_option,
                    entity: return_entity.unwrap(),
                    held_entity_option: return_entity,
                },
                builder: entity_type,
            });
        }
        None => {
            return_entity = None;
        }
    }

    return_entity
}
*/
