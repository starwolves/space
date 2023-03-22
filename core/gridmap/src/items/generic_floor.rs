use bevy::prelude::{AssetServer, Res, ResMut};
use entity::examine::RichName;
use resources::is_server::is_server;

use crate::{
    grid::{CellType, CellTypeName, Gridmap, TileProperties},
    init::InitTileProperties,
};

pub(crate) fn init_floor_properties(
    gridmap_data: Res<Gridmap>,
    assets: Res<AssetServer>,
    mut init: ResMut<InitTileProperties>,
) {
    let mesh_option;
    if !is_server() {
        mesh_option = Some(assets.load("models/floor/floor.glb#Scene0"));
    } else {
        mesh_option = None;
    }
    init.properties.push(TileProperties {
        id: *gridmap_data
            .main_name_id_map
            .get(&CellTypeName("generic_floor_1".to_string()))
            .unwrap(),
        name: RichName {
            name: "aluminum floor".to_string(),
            n: true,
            the: false,
        },
        description: "A generic floor tile.".to_string(),
        constructable: true,
        floor_cell: true,
        mesh_option,
        cell_type: CellType::Floor,
        ..Default::default()
    });
}
