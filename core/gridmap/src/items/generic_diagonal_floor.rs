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

pub(crate) fn init_generic_diagonal_floor(
    mut init: ResMut<InitTileProperties>,
    meshes: Res<GenericMeshes>,
) {
    let mesh_option: Option<Handle<GltfMesh>>;
    if !is_server() {
        mesh_option = Some(meshes.diagonal_template.clone());
    } else {
        mesh_option = None;
    }

    init.properties.push(TileProperties {
        name_id: CellTypeName("generic_diagonal_floor".to_string()),
        name: RichName {
            name: "diagonal aluminum floor".to_string(),
            n: true,
            the: false,
        },
        description: "A generic diagonal floor tile.".to_string(),
        constructable: true,
        floor_cell: true,
        mesh_option,
        cell_type: CellType::Diagonal,
        ..Default::default()
    });
}
