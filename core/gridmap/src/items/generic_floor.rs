use bevy::{
    gltf::GltfMesh,
    prelude::{AssetServer, Assets, Handle, Res, ResMut, Resource, StandardMaterial},
};
use bevy_xpbd_3d::prelude::Collider;
use entity::examine::RichName;
use resources::modes::{is_server, AppMode};

use crate::{
    grid::{CellType, CellTypeName, TileProperties},
    init::InitTileProperties,
};

use super::generic_assets::GenericMeshes;

#[derive(Default, Resource)]
pub struct GenericFloorMaterial {
    pub material_handle: Handle<StandardMaterial>,
}

pub(crate) fn init_generic_floor_material(
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut res: ResMut<GenericFloorMaterial>,
) {
    let albedo_texture_handle =
        asset_server.load("gridmap/floor_template/generic/generic_floor_base.png");
    let metallic_roughness_texture_handle =
        asset_server.load("gridmap/floor_template/generic/generic_floor_metal_rough.png");

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(albedo_texture_handle),
        metallic_roughness_texture: Some(metallic_roughness_texture_handle),
        perceptual_roughness: 0.9,
        metallic: 0.97,
        ..Default::default()
    });
    res.material_handle = material_handle;
}

pub(crate) fn init_generic_floor(
    mut init: ResMut<InitTileProperties>,
    meshes: Res<GenericMeshes>,
    mat: Res<GenericFloorMaterial>,
    app_mode: Res<AppMode>,
) {
    let mesh_option: Option<Handle<GltfMesh>>;
    let material_option;

    if !is_server() || matches!(*app_mode, AppMode::Correction) {
        mesh_option = Some(meshes.floor.clone_weak());
        material_option = Some(mat.material_handle.clone_weak());
    } else {
        mesh_option = None;
        material_option = None;
    }

    init.properties.push(TileProperties {
        name_id: CellTypeName("generic_floor".to_string()),
        name: RichName {
            name: "aluminum floor".to_string(),
            n: true,
            the: false,
        },
        description: "A generic floor tile.".to_string(),
        constructable: true,
        mesh_option,
        cell_type: CellType::Floor,
        material_option,
        collider: Collider::cuboid(1., 0.2, 1.),
        ..Default::default()
    });
}
