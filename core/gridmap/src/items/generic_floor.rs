use bevy::{
    gltf::GltfMesh,
    prelude::{AssetServer, Handle, Res, ResMut},
};
use entity::examine::RichName;
use resources::is_server::is_server;

use crate::{
    grid::{CellType, CellTypeName, TileProperties},
    init::InitTileProperties,
};

pub(crate) fn init_floor_properties(
    assets: Res<AssetServer>,
    mut init: ResMut<InitTileProperties>,
) {
    let mesh_option: Option<Handle<GltfMesh>>;
    if !is_server() {
        mesh_option = Some(assets.load("models/floor/floor.glb#Mesh0"));
    } else {
        mesh_option = None;
    }

    init.properties.push(TileProperties {
        name_id: CellTypeName("generic_floor_1".to_string()),
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
