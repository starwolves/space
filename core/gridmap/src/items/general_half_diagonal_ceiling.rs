use std::collections::HashMap;

use bevy::{
    gltf::GltfMesh,
    prelude::{Handle, Res, ResMut},
};
use entity::examine::RichName;
use resources::{grid::CellFace, is_server::is_server, math::Vec3Int};

use crate::{
    grid::{CellType, CellTypeName, FullCell, Gridmap, GroupTypeName, TileGroup, TileProperties},
    init::{InitTileGroups, InitTileProperties},
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
        cell_type: CellType::Center,
        vertical_rotation: false,
        x_rotations: vec![0, 16, 8, 20],
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
        cell_type: CellType::Center,
        vertical_rotation: false,
        x_rotations: vec![0, 16, 8, 20],
        ..Default::default()
    });
}
pub(crate) fn init_generic_half_diagonal_ceiling_group(
    gridmap_data: Res<Gridmap>,
    mut groups: ResMut<InitTileGroups>,
) {
    let mut wall_group = HashMap::new();
    wall_group.insert(
        Vec3Int { x: 0, y: 0, z: 0 },
        FullCell {
            face: CellFace::default(),
            orientation: 0,
            tile_type: *gridmap_data
                .main_name_id_map
                .get(&CellTypeName(
                    "generic_half_diagonal_ceiling_high".to_string(),
                ))
                .unwrap(),
            entity_option: None,
        },
    );
    wall_group.insert(
        Vec3Int { x: 1, y: 0, z: 0 },
        FullCell {
            face: CellFace::default(),
            orientation: 0,
            tile_type: *gridmap_data
                .main_name_id_map
                .get(&CellTypeName(
                    "generic_half_diagonal_ceiling_low".to_string(),
                ))
                .unwrap(),
            entity_option: None,
        },
    );

    groups.groups.push(TileGroup {
        name_id: GroupTypeName("generic_half_diagonal_ceiling_group".to_string()),
        map: wall_group,
    });
}
