use bevy::{
    gltf::GltfMesh,
    prelude::{Handle, Res, ResMut},
};
use entity::examine::RichName;
use resources::is_server::is_server;

use crate::{
    grid::{CellType, CellTypeName, TileProperties},
    init::InitTileProperties,
};

use super::generic_assets::GenericMeshes;

pub(crate) fn init_floor_properties(
    mut init: ResMut<InitTileProperties>,
    meshes: Res<GenericMeshes>,
) {
    let mesh_option: Option<Handle<GltfMesh>>;
    if !is_server() {
        mesh_option = Some(meshes.floor.clone());
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
