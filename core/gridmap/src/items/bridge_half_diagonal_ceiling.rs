use std::{collections::HashMap, f32::consts::PI};

use bevy::{
    gltf::GltfMesh,
    prelude::{
        AssetServer, Assets, Handle, Quat, Res, ResMut, Resource, StandardMaterial, Transform, Vec3,
    },
};
use bevy_xpbd_3d::prelude::Collider;
use entity::examine::RichName;
use resources::{
    grid::CellFace,
    math::Vec3Int,
    modes::{is_server, Mode},
};

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
        asset_server.load("gridmap/half_diagonal_template/bridge/high_base.png");
    let metallic_roughness_texture_handle =
        asset_server.load("gridmap/half_diagonal_template/bridge/high_metal_rough.png");

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(albedo_texture_handle.clone()),
        metallic_roughness_texture: Some(metallic_roughness_texture_handle.clone()),
        perceptual_roughness: 0.9,
        metallic: 0.97,
        ..Default::default()
    });
    res.high_material_handle = material_handle;

    let albedo_texture_handle =
        asset_server.load("gridmap/half_diagonal_template/bridge/low_base.png");
    let metallic_roughness_texture_handle =
        asset_server.load("gridmap/half_diagonal_template/bridge/low_metal_rough.png");

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
    app_mode: Res<Mode>,
) {
    let mesh_option: Option<Handle<GltfMesh>>;
    let material_option;

    if !is_server() || matches!(*app_mode, Mode::Correction) {
        mesh_option = Some(meshes.half_diagonal_template_low.clone());
        material_option = Some(mat.low_material_handle.clone());
    } else {
        mesh_option = None;
        material_option = None;
    }

    let mut rot = Quat::from_axis_angle(Vec3::new(1., 0., 0.), 0.5 * PI);
    rot *= Quat::from_axis_angle(Vec3::new(0., 1., 0.), -0.15 * PI);

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
        collider: Collider::cuboid(1.117, 1., 0.265),
        collider_position: Transform {
            rotation: rot,
            translation: Vec3::new(-0.05, -0.272, 0.),
            ..Default::default()
        },
        ..Default::default()
    });
}
pub(crate) fn init_bridge_half_diagonal_ceiling_high(
    mut init: ResMut<InitTileProperties>,
    meshes: Res<GenericMeshes>,
    mat: Res<BridgeHalfDiagonalCeilingMaterial>,
    app_mode: Res<Mode>,
) {
    let mesh_option: Option<Handle<GltfMesh>>;
    let material_option;

    if !is_server() || matches!(*app_mode, Mode::Correction) {
        mesh_option = Some(meshes.half_diagonal_template_high.clone());
        material_option = Some(mat.high_material_handle.clone());
    } else {
        mesh_option = None;
        material_option = None;
    }
    let mut rot = Quat::from_axis_angle(Vec3::new(1., 0., 0.), 0.5 * PI);
    rot *= Quat::from_axis_angle(Vec3::new(0., 1., 0.), -0.145 * PI);

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
        collider: Collider::cuboid(1.12, 1., 0.265),
        collider_position: Transform {
            rotation: rot,
            translation: Vec3::new(-0.05, 0.23, 0.),
            ..Default::default()
        },
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
