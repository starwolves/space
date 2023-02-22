use bevy::prelude::{warn, EventWriter, Query, Res};
use entity::spawn::ClientEntityServerEntity;
use gridmap::select_cell_yplane::{SelectCellCameraState, ShowYLevelPlane};
use inventory::server::inventory::Inventory;

use crate::construction_tool::ConstructionTool;
pub(crate) fn construction_tool_enable_select_cell_in_front_camera(
    inventory: Res<Inventory>,
    construction_tool_query: Query<&ConstructionTool>,
    map: Res<ClientEntityServerEntity>,
    state: Res<SelectCellCameraState>,
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
            if !state.y_plane_shown {
                yplane.send(ShowYLevelPlane { show: true });
            }
        }
        Err(_) => {
            return;
        }
    }
}
