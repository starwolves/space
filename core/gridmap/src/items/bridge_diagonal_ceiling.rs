use std::collections::HashMap;

use bevy::{
    gltf::GltfMesh,
    prelude::{AssetServer, Assets, Handle, Res, ResMut, Resource, StandardMaterial},
};
use entity::examine::RichName;
use resources::{grid::CellFace, is_server::is_server, math::Vec3Int};

use crate::{
    grid::{CellType, CellTypeName, FullCell, Gridmap, GroupTypeName, TileGroup, TileProperties},
    init::{InitTileGroups, InitTileProperties},
};

use super::generic_assets::GenericMeshes;

#[derive(Default, Resource)]
pub struct BridgeHalfDiagonalCeilingMaterial {
    pub high_material_handle: Handle<StandardMaterial>,
    pub low_material_handle: Handle<StandardMaterial>,
}

pub(crate) fn init_bridge_half_diagonal_ceiling_material(
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut res: ResMut<BridgeHalfDiagonalCeilingMaterial>,
) {
    let albedo_texture_handle =
        asset_server.load("models/half_diagonal_template/bridge/high_base.png");
    let metallic_roughness_texture_handle =
        asset_server.load("models/half_diagonal_template/bridge/high_metal_rough.png");

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(albedo_texture_handle.clone()),
        metallic_roughness_texture: Some(metallic_roughness_texture_handle.clone()),
        perceptual_roughness: 0.9,
        metallic: 0.97,
        ..Default::default()
    });
    res.high_material_handle = material_handle;

    let albedo_texture_handle =
        asset_server.load("models/half_diagonal_template/bridge/low_base.png");
    let metallic_roughness_texture_handle =
        asset_server.load("models/half_diagonal_template/bridge/low_metal_rough.png");

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(albedo_texture_handle.clone()),
        metallic_roughness_texture: Some(metallic_roughness_texture_handle.clone()),
        perceptual_roughness: 0.9,
        metallic: 0.97,
        ..Default::default()
    });
    res.low_material_handle = material_handle;
}

pub(crate) fn init_bridge_half_diagonal_ceiling_low(
    mut init: ResMut<InitTileProperties>,
    meshes: Res<GenericMeshes>,
    mat: Res<BridgeHalfDiagonalCeilingMaterial>,
) {
    let mesh_option: Option<Handle<GltfMesh>>;
    let material_option;

    if !is_server() {
        mesh_option = Some(meshes.half_diagonal_template_low.clone());
        material_option = Some(mat.low_material_handle.clone());
    } else {
        mesh_option = None;
        material_option = None;
    }

    init.properties.push(TileProperties {
        name_id: CellTypeName("bridge_half_diagonal_ceiling_low".to_string()),
        name: RichName {
            name: "diagonal aluminum ceiling".to_string(),
            n: true,
            the: false,
        },
        description: "A bridge ceiling tile.".to_string(),
        constructable: false,
        floor_cell: true,
        mesh_option,
        cell_type: CellType::Center,
        vertical_rotation: false,
        x_rotations: vec![0, 16, 10, 22],
        material_option,
        ..Default::default()
    });
}
pub(crate) fn init_bridge_half_diagonal_ceiling_high(
    mut init: ResMut<InitTileProperties>,
    meshes: Res<GenericMeshes>,
    mat: Res<BridgeHalfDiagonalCeilingMaterial>,
) {
    let mesh_option: Option<Handle<GltfMesh>>;
    let material_option;

    if !is_server() {
        mesh_option = Some(meshes.half_diagonal_template_high.clone());
        material_option = Some(mat.high_material_handle.clone());
    } else {
        mesh_option = None;
        material_option = None;
    }

    init.properties.push(TileProperties {
        name_id: CellTypeName("bridge_half_diagonal_ceiling_high".to_string()),
        name: RichName {
            name: "diagonal aluminum ceiling".to_string(),
            n: true,
            the: false,
        },
        description: "A bridge ceiling tile.".to_string(),
        constructable: false,
        floor_cell: true,
        mesh_option,
        cell_type: CellType::Center,
        vertical_rotation: false,
        x_rotations: vec![0, 16, 10, 22],
        material_option,
        ..Default::default()
    });
}
pub(crate) fn init_bridge_half_diagonal_ceiling_group(
    gridmap_data: Res<Gridmap>,
    mut groups: ResMut<InitTileGroups>,
) {
    let mut wall_group = HashMap::new();
    wall_group.insert(
        Vec3Int { x: -1, y: 0, z: 0 },
        FullCell {
            face: CellFace::default(),
            orientation: 0,
            tile_type: *gridmap_data
                .main_name_id_map
                .get(&CellTypeName(
                    "bridge_half_diagonal_ceiling_high".to_string(),
                ))
                .unwrap(),
            entity_option: None,
        },
    );
    wall_group.insert(
        Vec3Int { x: 0, y: 0, z: 0 },
        FullCell {
            face: CellFace::default(),
            orientation: 0,
            tile_type: *gridmap_data
                .main_name_id_map
                .get(&CellTypeName(
                    "bridge_half_diagonal_ceiling_low".to_string(),
                ))
                .unwrap(),
            entity_option: None,
        },
    );

    groups.groups.push(TileGroup {
        name_id: GroupTypeName("bridge_half_diagonal_ceiling_group".to_string()),
        map: wall_group,
    });
}
