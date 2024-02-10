use std::{fs, path::Path};

use bevy::log::info;
use bevy::log::warn;
use bevy::prelude::{Commands, EventWriter, Res, ResMut, Resource};
use resources::grid::CellFace;
use resources::math::Vec3Int;

use crate::grid::{
    AddTile, CellTypeId, CellTypeName, Gridmap, GroupTypeId, GroupTypeName, TileGroup,
    TileProperties,
};

/// Initiate map resource meta-data.

#[derive(Default, Resource)]
pub struct InitTileProperties {
    pub properties: Vec<TileProperties>,
}

#[derive(Default, Resource)]
pub struct InitTileGroups {
    pub groups: Vec<TileGroup>,
}

pub(crate) fn init_tile_properties(mut gridmap: ResMut<Gridmap>, init: Res<InitTileProperties>) {
    let mut current_map_mainordered_cells_typed = vec![];
    let mut properties_ordered = init.properties.clone();
    properties_ordered.sort_by(|a, b: &TileProperties| a.name_id.cmp(&b.name_id));
    for properties in properties_ordered.iter() {
        let gri_id = CellTypeId(gridmap.tile_type_incremental);
        gridmap.tile_properties.insert(gri_id, properties.clone());

        gridmap
            .main_name_id_map
            .insert(properties.name_id.clone(), gri_id);
        gridmap
            .main_id_name_map
            .insert(gri_id, properties.name_id.clone());
        current_map_mainordered_cells_typed.push(properties.name_id.clone());

        gridmap.tile_type_incremental += 1;
    }
    gridmap.ordered_main_names = current_map_mainordered_cells_typed;
    info!("Loaded {} gridmap cell types.", init.properties.len());
}

pub(crate) fn init_tile_groups(mut gridmap: ResMut<Gridmap>, init_group: Res<InitTileGroups>) {
    let mut groups_ordered = init_group.groups.clone();
    groups_ordered.sort_by(|a, b: &TileGroup| a.name_id.cmp(&b.name_id));

    for groups in groups_ordered.iter() {
        let group_id = GroupTypeId(gridmap.group_type_incremental);
        gridmap.groups.insert(group_id, groups.map.clone());

        gridmap
            .group_id_map
            .insert(groups.name_id.clone(), group_id);
        gridmap
            .id_group_map
            .insert(group_id, groups.name_id.clone());

        gridmap.group_type_incremental += 1;
    }

    info!("Loaded {} gridmap group types.", groups_ordered.len());
}

/// Build the gridmaps in their own resources from ron.

pub(crate) fn load_ron_gridmap(
    gridmap: Res<Gridmap>,
    mut set_cell: EventWriter<AddTile>,
    mut commands: Commands,
) {
    // Load map json data into real static bodies.
    let main_ron = Path::new("data")
        .join("maps")
        .join("bullseye")
        .join("main.bin");
    let current_map_main_raw_ron = fs::read(main_ron)
        .expect("startup_build_map() Error reading map main.ron file from drive.");

    if current_map_main_raw_ron.len() == 0 {
        warn!("Empty main.ron map file.");
        return;
    }

    let current_map_main_data: Vec<CellDataExport> =
        bincode::deserialize(&current_map_main_raw_ron)
            .expect("startup_build_map() Error parsing map main.ron String.");

    for cell_data in current_map_main_data.iter() {
        match &cell_data.item {
            ItemExport::Cell(item) => {
                let cell_item_id;

                match gridmap.main_name_id_map.get(item) {
                    Some(x) => {
                        cell_item_id = *x;
                    }
                    None => {
                        warn!("Couldnt find item {:?}", item);
                        break;
                    }
                };

                set_cell.send(AddTile {
                    id: cell_data.id,
                    face: cell_data.face.clone(),
                    orientation: cell_data.orientation.clone(),
                    tile_type: cell_item_id,
                    group_instance_id_option: None,
                    entity: commands.spawn(()).id(),
                    default_map_spawn: true,
                });
            }
            ItemExport::Group(item) => {
                let id;
                match gridmap.main_name_id_map.get(&item.cell) {
                    Some(n) => {
                        id = n;
                    }
                    None => {
                        warn!("couildnt find name");
                        continue;
                    }
                }

                set_cell.send(AddTile {
                    id: cell_data.id,
                    face: cell_data.face.clone(),
                    orientation: cell_data.orientation.clone(),
                    tile_type: *id,
                    group_instance_id_option: Some(item.group_id),
                    entity: commands.spawn(()).id(),
                    default_map_spawn: true,
                });
            }
        }
    }

    info!("Spawned {} map cells.", current_map_main_data.len());
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CellDataExport {
    pub id: Vec3Int,
    /// Cell item id.
    pub item: ItemExport,
    /// Cell rotation.
    pub orientation: u8,
    pub face: CellFace,
}

#[derive(Serialize, Deserialize)]
pub enum ItemExport {
    Cell(CellTypeName),
    Group(GroupItem),
}
#[derive(Serialize, Deserialize)]
pub struct GroupItem {
    pub name: GroupTypeName,
    pub group_id: u32,
    pub cell: CellTypeName,
}
