use bevy::{
    prelude::{AssetServer, Assets, Res, ResMut, StandardMaterial, Transform},
    render::color::Color,
};
use bevy_xpbd_3d::prelude::Collider;
use entity::examine::RichName;
use resources::modes::{is_server, AppMode};

use crate::{
    grid::{CellType, CellTypeName, TileProperties},
    init::InitTileProperties,
};

use super::{generic_assets::GenericMeshes, wall_flat::WallMaterials};
pub(crate) fn init_star_lights_material(
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut res: ResMut<WallMaterials>,
) {
    let albedo_texture_handle = asset_server.load("gridmap/star_lights/star_lights_base.png");
    let metallic_roughness_texture_handle =
        asset_server.load("gridmap/star_lights/star_lights_metal_rough.png");
    let emissive_texture_handle = asset_server.load("gridmap/star_lights/star_lights_base.png");

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(albedo_texture_handle),
        metallic_roughness_texture: Some(metallic_roughness_texture_handle),
        perceptual_roughness: 0.9,
        metallic: 0.97,
        emissive_texture: Some(emissive_texture_handle),
        emissive: Color::rgb(10000., 10000., 10000.),
        ..Default::default()
    });
    res.star_lights_handle = material_handle;
}

pub(crate) fn init_star_lights(
    mut init: ResMut<InitTileProperties>,
    meshes: Res<GenericMeshes>,
    mat: Res<WallMaterials>,
    app_mode: Res<AppMode>,
) {
    let mut default_isometry = Transform::IDENTITY;

    default_isometry.translation.y = -0.5;

    let mesh_option;
    let material_option;
    if !is_server() || matches!(*app_mode, AppMode::Correction) {
        mesh_option = Some(meshes.star_lights.clone_weak());

        material_option = Some(mat.star_lights_handle.clone_weak());
    } else {
        mesh_option = None;
        material_option = None;
    }
    init.properties.push(TileProperties {
        name_id: CellTypeName("star_lights".to_string()),
        name: RichName {
            name: "star lights".to_string(),
            n: true,
            the: false,
        },
        description: "A bunch of star lights.".to_string(),
        constructable: true,
        mesh_option,
        cell_type: CellType::WallDetail,
        material_option,
        collider: Collider::cuboid(1., 1., 0.2),
        x_rotations: vec![0, 16, 10, 22],
        is_detail: true,
        ..Default::default()
    });
}
