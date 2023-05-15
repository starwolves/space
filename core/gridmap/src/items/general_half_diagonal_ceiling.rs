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

pub(crate) fn init_generic_half_diagonal_ceiling_low(
    mut init: ResMut<InitTileProperties>,
    meshes: Res<GenericMeshes>,
) {
    let mesh_option: Option<Handle<GltfMesh>>;
    if !is_server() {
        mesh_option = Some(meshes.half_diagonal_template_low.clone());
    } else {
        mesh_option = None;
    }

    init.properties.push(TileProperties {
        name_id: CellTypeName("generic_half_diagonal_ceiling_low".to_string()),
        name: RichName {
            name: "diagonal aluminum ceiling".to_string(),
            n: true,
            the: false,
        },
        description: "A generic ceiling tile.".to_string(),
        constructable: false,
        floor_cell: true,
        mesh_option,
        cell_type: CellType::Diagonal,
        ..Default::default()
    });
}
pub(crate) fn init_generic_half_diagonal_ceiling_high(
    mut init: ResMut<InitTileProperties>,
    meshes: Res<GenericMeshes>,
) {
    let mesh_option: Option<Handle<GltfMesh>>;
    if !is_server() {
        mesh_option = Some(meshes.half_diagonal_template_high.clone());
    } else {
        mesh_option = None;
    }

    init.properties.push(TileProperties {
        name_id: CellTypeName("generic_half_diagonal_ceiling_high".to_string()),
        name: RichName {
            name: "diagonal aluminum ceiling".to_string(),
            n: true,
            the: false,
        },
        description: "A generic ceiling tile.".to_string(),
        constructable: false,
        floor_cell: true,
        mesh_option,
        cell_type: CellType::Diagonal,
        ..Default::default()
    });
}
