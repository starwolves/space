use api::{
    data::{ShowcaseData},
    humanoid::UsedNames,
    inventory::Inventory,
    rigid_body::RigidBodyLinkTransform,
};
use bevy::prelude::{
    warn, Commands, Entity, EventReader, EventWriter, Query, Res, ResMut, Transform,
};
use console_commands::{
    commands::{NetEntityConsole, CONSOLE_ERROR_COLOR},
    player_selectors::player_selector_to_entities,
};
use entity::{
    meta::EntityDataResource,
    spawn::{DefaultSpawnEvent, SpawnData, SpawnEvent},
};
use gridmap::{commands::rcon_spawn_entity, grid::GridmapMain};
use networking::messages::ReliableServerMessage;
use pawn::pawn::Pawn;
use server::core::HandleToEntity;

use crate::combat::{MeleeCombat, ProjectileCombat};

use super::item::InventoryItem;

/// Inventory item bundle.
pub struct InventoryItemBundle {
    pub inventory_item: InventoryItem,
    pub melee_combat: MeleeCombat,
    pub projectile_combat_option: Option<ProjectileCombat>,
}

/// Inventory item builder data.
pub struct InventoryBuilderData {
    pub inventory_item: InventoryItem,
    pub holder_entity_option: Option<Entity>,
    pub melee_combat: MeleeCombat,
    pub projectile_option: Option<ProjectileCombat>,
}

/// Build inventory item at summon stage.
pub(crate) fn inventory_item_builder(
    commands: &mut Commands,
    entity: Entity,
    data: InventoryBuilderData,
) {
    let mut builder = commands.entity(entity);
    builder.insert_bundle((data.inventory_item, data.melee_combat));
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
/// Inventory item summonable.
pub trait InventoryItemSummonable {
    fn get_bundle(&self, spawn_data: &SpawnData) -> InventoryItemBundle;
}

/// Inventory item spawn handler.
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
                melee_combat: inventory_item_bundle.melee_combat,
                projectile_option: inventory_item_bundle.projectile_combat_option,
            },
        );
    }
}

/// Function to spawn an entity in another entity's inventory through an RCON command.
pub fn rcon_spawn_held_entity(
    entity_name: String,
    target_selector: String,
    mut commands: &mut Commands,
    command_executor_entity: Entity,
    command_executor_handle_option: Option<u64>,
    mut net_console_commands: &mut EventWriter<NetEntityConsole>,
    player_inventory_query: &mut Query<&mut Inventory>,
    mut rigid_body_positions: &mut Query<(&Transform, &Pawn)>,
    gridmap_main: &Res<GridmapMain>,
    mut used_names: &mut ResMut<UsedNames>,
    handle_to_entity: &Res<HandleToEntity>,
    entity_data: &mut ResMut<EntityDataResource>,
    default_spawner: &mut EventWriter<DefaultSpawnEvent>,
) {
    for target_entity in player_selector_to_entities(
        command_executor_entity,
        command_executor_handle_option,
        &target_selector,
        used_names,
        net_console_commands,
    )
    .iter()
    {
        let mut player_inventory;

        match player_inventory_query.get_mut(*target_entity) {
            Ok(inventory) => {
                player_inventory = inventory;
            }
            Err(_rr) => {
                match command_executor_handle_option {
                    Some(t) => {
                        net_console_commands.send(NetEntityConsole {
                            handle: t,
                            message: ReliableServerMessage::ConsoleWriteLine(
                                "[color=".to_string() + CONSOLE_ERROR_COLOR + "]An error occured when executing your command, please report this.[/color]"
                            ),
                        });
                    }
                    None => {}
                }
                warn!("spawn_held_entity console command couldn't find inventory component beloning to player target.");

                continue;
            }
        }

        let player_handle;

        match handle_to_entity.inv_map.get(target_entity) {
            Some(handle) => {
                player_handle = *handle;
            }
            None => {
                match command_executor_handle_option {
                    Some(t) => {
                        net_console_commands.send(NetEntityConsole {
                            handle: t,
                            message: ReliableServerMessage::ConsoleWriteLine(
                                "[color=".to_string() + CONSOLE_ERROR_COLOR + "]An error occured when executing your command, please report this.[/color]"
                            ),
                        });
                    }
                    None => {}
                }

                warn!("spawn_held_entity console command couldn't find handle belonging to target entity.");
                continue;
            }
        }

        let mut available_slot = None;

        for slot in player_inventory.slots.iter_mut() {
            let is_hand = matches!(slot.slot_name.as_str(), "left_hand" | "right_hand");
            if is_hand && slot.slot_item.is_none() {
                available_slot = Some(slot);
            }
        }

        match available_slot {
            Some(slot) => {
                let entity_option = spawn_held_entity(
                    entity_name.clone(),
                    commands,
                    command_executor_entity,
                    None,
                    &entity_data,
                    default_spawner,
                );

                match entity_option {
                    Some(entity) => {
                        slot.slot_item = Some(entity);

                        net_console_commands.send(NetEntityConsole {
                            handle: player_handle,
                            message: ReliableServerMessage::PickedUpItem(
                                entity_name.clone(),
                                entity.to_bits(),
                                slot.slot_name.clone(),
                            ),
                        });

                        net_console_commands.send(NetEntityConsole {
                            handle: player_handle,
                            message: ReliableServerMessage::ChatMessage(
                                "A new entity has appeared in your hand.".to_string(),
                            ),
                        });
                    }
                    None => match command_executor_handle_option {
                        Some(t) => {
                            net_console_commands.send(NetEntityConsole {
                                handle: t,
                                message: ReliableServerMessage::ConsoleWriteLine(
                                    "[color=".to_string()
                                        + CONSOLE_ERROR_COLOR
                                        + "]Unknown entity name \""
                                        + &entity_name
                                        + "\" was provided.[/color]",
                                ),
                            });
                        }
                        None => {}
                    },
                }
            }
            None => {
                rcon_spawn_entity(
                    entity_name.clone(),
                    target_selector.clone(),
                    1,
                    &mut commands,
                    command_executor_entity,
                    command_executor_handle_option,
                    &mut rigid_body_positions,
                    &mut net_console_commands,
                    &gridmap_main,
                    &mut used_names,
                    handle_to_entity,
                    &entity_data,
                    default_spawner,
                );
            }
        }
    }
}

/// Function to spawn an entity that is held in someone's hands.
pub fn spawn_held_entity(
    entity_name: String,
    commands: &mut Commands,
    holder_entity: Entity,
    showcase_handle_option: Option<ShowcaseData>,
    entity_data: &ResMut<EntityDataResource>,
    default_spawner: &mut EventWriter<DefaultSpawnEvent>,
) -> Option<Entity> {
    let return_entity;

    match entity_data.name_to_id.get(&entity_name) {
        Some(_id) => {
            return_entity = Some(commands.spawn().id());

            default_spawner.send(DefaultSpawnEvent {
                spawn_data: SpawnData {
                    entity_transform: Transform::identity(),
                    correct_transform: false,
                    holder_entity_option: Some(holder_entity),
                    default_map_spawn: false,
                    raw_entity_option: None,
                    showcase_data_option: showcase_handle_option,
                    entity_name,
                    entity: return_entity.unwrap(),
                    held_entity_option: return_entity,
                },
            });
        }
        None => {
            return_entity = None;
        }
    }

    return_entity
}
