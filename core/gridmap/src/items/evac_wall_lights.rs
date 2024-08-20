use bevy::{
    color::Color,
    prelude::{AssetServer, Assets, Res, ResMut, StandardMaterial, Transform},
};
use bevy_xpbd_3d::prelude::Collider;
use entity::examine::RichName;
use resources::modes::{is_server, AppMode};

use crate::{
    grid::{CellType, CellTypeName, TileProperties},
    init::InitTileProperties,
};

use super::{generic_assets::GenericMeshes, wall_flat::WallMaterials};
pub(crate) fn init_evac_wall_lights_material(
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut res: ResMut<WallMaterials>,
) {
    let albedo_texture_handle =
        asset_server.load("gridmap/wall_evac_lights/wall_evac_lights_base.png");
    let metallic_roughness_texture_handle =
        asset_server.load("gridmap/wall_evac_lights/wall_evac_lights_metal_rough.png");
    let emissive_texture_handle =
        asset_server.load("gridmap/wall_evac_lights/wall_evac_lights_emissive.png");

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(albedo_texture_handle),
        metallic_roughness_texture: Some(metallic_roughness_texture_handle),
        perceptual_roughness: 0.9,
        emissive_texture: Some(emissive_texture_handle),
        emissive: Color::srgb(50000., 50000., 50000.).into(),
        metallic: 0.97,
        ..Default::default()
    });
    res.evac_lights = material_handle;
}

pub(crate) fn init_evec_wall_lights(
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
        mesh_option = Some(meshes.wall_lights.clone_weak());

        material_option = Some(mat.evac_lights.clone_weak());
    } else {
        mesh_option = None;
        material_option = None;
    }
    init.properties.push(TileProperties {
        name_id: CellTypeName("wall_evac_lights".to_string()),
        name: RichName {
            name: "evac aluminum wall".to_string(),
            n: true,
            the: false,
        },
        description: "A clean evac wall tile with lights.".to_string(),
        constructable: true,
        mesh_option,
        cell_type: CellType::Wall,
        material_option,
        collider: Collider::cuboid(1., 1., 0.2),
        ..Default::default()
    });
}
