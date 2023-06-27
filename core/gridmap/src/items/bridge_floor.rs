use bevy::{
    gltf::GltfMesh,
    prelude::{AssetServer, Assets, Handle, Res, ResMut, Resource, StandardMaterial},
};
use entity::examine::RichName;
use resources::is_server::is_server;

use crate::{
    grid::{CellType, CellTypeName, TileProperties},
    init::InitTileProperties,
};

use super::generic_assets::GenericMeshes;

#[derive(Default, Resource)]
pub struct BridgeFloorMaterial {
    pub filled_handle: Handle<StandardMaterial>,
    pub half_handle: Handle<StandardMaterial>,
    pub corner_handle: Handle<StandardMaterial>,
    pub corner2_handle: Handle<StandardMaterial>,
}

pub(crate) fn init_bridge_floor_material(
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut res: ResMut<BridgeFloorMaterial>,
) {
    let albedo_texture_handle =
        asset_server.load("models/floor_template/bridge/filled_floor_base.png");
    let metallic_roughness_texture_handle =
        asset_server.load("models/floor_template/generic/generic_floor_metal_rough.png");

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(albedo_texture_handle.clone()),
        metallic_roughness_texture: Some(metallic_roughness_texture_handle.clone()),

        perceptual_roughness: 0.2,
        metallic: 0.91,
        ..Default::default()
    });
    res.filled_handle = material_handle;

    let albedo_texture_handle =
        asset_server.load("models/floor_template/bridge/half_floor_base.png");
    let metallic_roughness_texture_handle =
        asset_server.load("models/floor_template/generic/generic_floor_metal_rough.png");

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(albedo_texture_handle.clone()),
        metallic_roughness_texture: Some(metallic_roughness_texture_handle.clone()),

        perceptual_roughness: 0.2,
        metallic: 0.91,
        ..Default::default()
    });
    res.half_handle = material_handle;

    let albedo_texture_handle =
        asset_server.load("models/floor_template/bridge/corner_floor_base.png");
    let metallic_roughness_texture_handle =
        asset_server.load("models/floor_template/generic/generic_floor_metal_rough.png");

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(albedo_texture_handle.clone()),
        metallic_roughness_texture: Some(metallic_roughness_texture_handle.clone()),

        perceptual_roughness: 0.2,
        metallic: 0.91,
        ..Default::default()
    });
    res.corner_handle = material_handle;

    let albedo_texture_handle =
        asset_server.load("models/floor_template/bridge/corner2_floor_base.png");
    let metallic_roughness_texture_handle =
        asset_server.load("models/floor_template/generic/generic_floor_metal_rough.png");

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(albedo_texture_handle.clone()),
        metallic_roughness_texture: Some(metallic_roughness_texture_handle.clone()),
        perceptual_roughness: 0.2,
        metallic: 0.91,
        ..Default::default()
    });
    res.corner2_handle = material_handle;
}

pub(crate) fn init_filled_bridge_floor(
    mut init: ResMut<InitTileProperties>,
    meshes: Res<GenericMeshes>,
    mat: Res<BridgeFloorMaterial>,
) {
    let mesh_option: Option<Handle<GltfMesh>>;
    let material_option;

    if !is_server() {
        mesh_option = Some(meshes.floor.clone());
        material_option = Some(mat.filled_handle.clone());
    } else {
        mesh_option = None;
        material_option = None;
    }

    init.properties.push(TileProperties {
        name_id: CellTypeName("filled_bridge_floor".to_string()),
        name: RichName {
            name: "aluminum bridge floor".to_string(),
            n: true,
            the: false,
        },
        description: "A bridge floor tile.".to_string(),
        constructable: true,
        floor_cell: true,
        mesh_option,
        cell_type: CellType::Floor,
        material_option,
        ..Default::default()
    });
}

pub(crate) fn init_half_bridge_floor(
    mut init: ResMut<InitTileProperties>,
    meshes: Res<GenericMeshes>,
    mat: Res<BridgeFloorMaterial>,
) {
    let mesh_option: Option<Handle<GltfMesh>>;
    let material_option;

    if !is_server() {
        mesh_option = Some(meshes.floor.clone());
        material_option = Some(mat.half_handle.clone());
    } else {
        mesh_option = None;
        material_option = None;
    }

    init.properties.push(TileProperties {
        name_id: CellTypeName("half_bridge_floor".to_string()),
        name: RichName {
            name: "aluminum bridge floor".to_string(),
            n: true,
            the: false,
        },
        description: "A bridge floor tile.".to_string(),
        constructable: true,
        floor_cell: true,
        mesh_option,
        cell_type: CellType::Floor,
        material_option,
        ..Default::default()
    });
}

pub(crate) fn init_corner_bridge_floor(
    mut init: ResMut<InitTileProperties>,
    meshes: Res<GenericMeshes>,
    mat: Res<BridgeFloorMaterial>,
) {
    let mesh_option: Option<Handle<GltfMesh>>;
    let material_option;

    if !is_server() {
        mesh_option = Some(meshes.floor.clone());
        material_option = Some(mat.corner_handle.clone());
    } else {
        mesh_option = None;
        material_option = None;
    }

    init.properties.push(TileProperties {
        name_id: CellTypeName("corner_bridge_floor".to_string()),
        name: RichName {
            name: "aluminum bridge floor".to_string(),
            n: true,
            the: false,
        },
        description: "A bridge floor tile.".to_string(),
        constructable: true,
        floor_cell: true,
        mesh_option,
        cell_type: CellType::Floor,
        material_option,
        ..Default::default()
    });
}
pub(crate) fn init_corner2_bridge_floor(
    mut init: ResMut<InitTileProperties>,
    meshes: Res<GenericMeshes>,
    mat: Res<BridgeFloorMaterial>,
) {
    let mesh_option: Option<Handle<GltfMesh>>;
    let material_option;

    if !is_server() {
        mesh_option = Some(meshes.floor.clone());
        material_option = Some(mat.corner2_handle.clone());
    } else {
        mesh_option = None;
        material_option = None;
    }

    init.properties.push(TileProperties {
        name_id: CellTypeName("corner2_bridge_floor".to_string()),
        name: RichName {
            name: "aluminum bridge floor".to_string(),
            n: true,
            the: false,
        },
        description: "A bridge floor tile.".to_string(),
        constructable: true,
        floor_cell: true,
        mesh_option,
        cell_type: CellType::Floor,
        material_option,
        ..Default::default()
    });
}
