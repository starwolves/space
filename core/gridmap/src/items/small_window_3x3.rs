use bevy::{
    asset::{AssetServer, Assets},
    ecs::system::{Res, ResMut},
    pbr::StandardMaterial,
    prelude::AlphaMode,
    transform::components::Transform,
};
use bevy_xpbd_3d::plugins::collision::Collider;
use entity::examine::RichName;
use resources::modes::{is_server, AppMode};

use crate::{
    grid::{CellType, CellTypeName, TileProperties},
    init::InitTileProperties,
};

use super::{generic_assets::GenericMeshes, large_window_3x3::LargeWindowMaterials};
pub(crate) fn init_small_window_3x3_material(
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut res: ResMut<LargeWindowMaterials>,
) {
    let albedo_texture_handle = asset_server.load("gridmap/small_windows/3x3/window_base.png");
    let metallic_roughness_texture_handle =
        asset_server.load("gridmap/small_windows/3x3/window_metal_rough.png");

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(albedo_texture_handle),
        metallic_roughness_texture: Some(metallic_roughness_texture_handle),
        perceptual_roughness: 0.9,
        metallic: 0.9,
        alpha_mode: AlphaMode::Blend,
        thickness: 0.2,
        ior: 1.52,
        diffuse_transmission: 1.,
        ..Default::default()
    });
    res.small_3x3 = material_handle;
}

pub(crate) fn init_small_window_3x3(
    mut init: ResMut<InitTileProperties>,
    meshes: Res<GenericMeshes>,
    mat: Res<LargeWindowMaterials>,
    app_mode: Res<AppMode>,
) {
    let mut default_isometry = Transform::IDENTITY;

    default_isometry.translation.y = -0.5;

    let mesh_option;
    let material_option;
    if !is_server() || matches!(*app_mode, AppMode::Correction) {
        mesh_option = Some(meshes.small_window_3x3.clone_weak());

        material_option = Some(mat.small_3x3.clone_weak());
    } else {
        mesh_option = None;
        material_option = None;
    }
    init.properties.push(TileProperties {
        name_id: CellTypeName("small_window_3x3".to_string()),
        name: RichName {
            name: "small window".to_string(),
            n: true,
            the: false,
        },
        description: "A small window.".to_string(),
        constructable: true,
        mesh_option,
        cell_type: CellType::Wall,
        material_option,
        collider: Collider::cuboid(3., 3., 0.2),
        ..Default::default()
    });
}
