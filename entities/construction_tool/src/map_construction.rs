use bevy::prelude::{warn, Commands, EventReader, EventWriter, Query, Res};
use entity::spawn::ClientEntityServerEntity;
use gridmap::{
    construction::{GridmapConstructionState, ShowYLevelPlane},
    grid::{AddTile, Gridmap, RemoveTile},
    net::GridmapClientMessage,
};
use inventory::server::inventory::Inventory;
use networking::server::{HandleToEntity, IncomingReliableClientMessage};

use crate::construction_tool::ConstructionTool;
pub(crate) fn construction_tool_enable_select_cell_in_front_camera(
    inventory: Res<Inventory>,
    construction_tool_query: Query<&ConstructionTool>,
    map: Res<ClientEntityServerEntity>,
    state: Res<GridmapConstructionState>,
    mut yplane: EventWriter<ShowYLevelPlane>,
) {
    let active_inventory_entity;

    match inventory.active_item {
        Some(active_inventory_item_server) => match map.map.get(&active_inventory_item_server) {
            Some(active_inventory_item) => {
                active_inventory_entity = *active_inventory_item;
            }
            None => {
                warn!("Couldnt get client entity from map.");
                return;
            }
        },
        None => {
            return;
        }
    }

    match construction_tool_query.get(active_inventory_entity) {
        Ok(_component) => {
            if !state.is_constructing {
                yplane.send(ShowYLevelPlane { show: true });
            }
        }
        Err(_) => {
            return;
        }
    }
}

pub(crate) fn mouse_click_input(
    mut net: EventReader<IncomingReliableClientMessage<GridmapClientMessage>>,
    inventory_query: Query<&Inventory>,
    handle_to_entity: Res<HandleToEntity>,
    construction_tool_query: Query<&ConstructionTool>,
    mut add_events: EventWriter<AddTile>,
    mut remove_events: EventWriter<RemoveTile>,
    mut commands: Commands,
    gridmap: Res<Gridmap>,
) {
    for message in net.iter() {
        let client_entity;
        match handle_to_entity.map.get(&message.handle) {
            Some(entity) => {
                client_entity = *entity;
            }
            None => {
                warn!("Couldnt get entity from map.");
                return;
            }
        }

        let active_item_entity;

        match inventory_query.get(client_entity) {
            Ok(inventory) => match inventory.active_item {
                Some(e) => {
                    active_item_entity = e;
                }
                None => {
                    return;
                }
            },
            Err(_) => {
                return;
            }
        }

        let construction_tool_component;

        match construction_tool_query.get(active_item_entity) {
            Ok(component) => {
                construction_tool_component = component;
            }
            Err(_) => {
                return;
            }
        }

        match &message.message {
            GridmapClientMessage::ConstructCells(construct) => {
                let type_id;

                match construction_tool_component.construction_option.clone() {
                    Some(i) => {
                        type_id = i;
                    }
                    None => {
                        return;
                    }
                }

                match type_id {
                    gridmap::grid::CellIds::CellType(id) => {
                        for cell in construct.cells.iter() {
                            add_events.send(AddTile {
                                id: cell.id,
                                tile_type: id,
                                orientation: cell.orientation,
                                face: cell.face.clone(),
                                group_id_option: None,
                                entity: commands.spawn(()).id(),
                                default_map_spawn: false,
                            });
                        }
                    }
                    gridmap::grid::CellIds::GroupType(id) => {
                        for cell in construct.cells.iter() {
                            match gridmap.groups.get(&id) {
                                Some(cells) => {
                                    for (local_id, cell_type) in cells.iter() {
                                        add_events.send(AddTile {
                                            id: *local_id + cell.id,
                                            tile_type: cell_type.tile_type,
                                            orientation: cell.orientation,
                                            face: cell.face.clone(),
                                            group_id_option: None,
                                            entity: commands.spawn(()).id(),
                                            default_map_spawn: false,
                                        });
                                    }
                                }
                                None => {
                                    warn!("Couldnt find group type.");
                                }
                            }
                        }
                    }
                }
            }
            GridmapClientMessage::DeconstructCells(deconstruct) => {
                for cell in deconstruct.cells.iter() {
                    remove_events.send(RemoveTile { cell: cell.clone() });
                }
            }
            _ => (),
        }
    }
}
