use bevy::{
    math::Vec3,
    pbr::AlphaMode,
    prelude::{AssetServer, Assets, Res, ResMut, StandardMaterial, Transform},
    render::color::Color,
};
use bevy_xpbd_3d::prelude::Collider;
use entity::examine::RichName;
use resources::{
    light::default_point_light,
    modes::{is_server, AppMode},
};

use crate::{
    grid::{CellType, CellTypeName, TileLight, TileProperties},
    init::InitTileProperties,
};

use super::{ceiling_light_bar::LightMaterials, generic_assets::GenericMeshes};
pub(crate) fn init_wall_light_bulb_material(
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut res: ResMut<LightMaterials>,
) {
    let albedo_texture_handle = asset_server.load("gridmap/wall_light_bulb/wall_light_base.png");
    let metallic_roughness_texture_handle =
        asset_server.load("gridmap/wall_light_bulb/wall_light_metal_rough.png");
    let emissive = asset_server.load("gridmap/wall_light_bulb/wall_light_emissive.png");
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(albedo_texture_handle),
        metallic_roughness_texture: Some(metallic_roughness_texture_handle),
        perceptual_roughness: 0.9,
        metallic: 0.97,
        alpha_mode: AlphaMode::Blend,
        emissive_texture: Some(emissive),
        emissive: Color::rgb(100000., 100000., 100000.),
        ..Default::default()
    });
    res.wall = material_handle;
}

pub(crate) fn init_wall_light_bulb(
    mut init: ResMut<InitTileProperties>,
    meshes: Res<GenericMeshes>,
    mat: Res<LightMaterials>,
    app_mode: Res<AppMode>,
) {
    let mut default_isometry = Transform::IDENTITY;

    default_isometry.translation.y = -0.5;

    let mesh_option;
    let material_option;
    if !is_server() || matches!(*app_mode, AppMode::Correction) {
        mesh_option = Some(meshes.wall_light.clone_weak());

        material_option = Some(mat.wall.clone_weak());
    } else {
        mesh_option = None;
        material_option = None;
    }
    init.properties.push(TileProperties {
        name_id: CellTypeName("wall_light_bulb".to_string()),
        name: RichName {
            name: "wall light bulb".to_string(),
            n: true,
            the: false,
        },
        description: "A cwall light bulb.".to_string(),
        constructable: true,
        mesh_option,
        cell_type: CellType::WallDetail,
        material_option,
        collider: Collider::cuboid(1., 1., 0.2),
        is_detail: true,
        x_rotations: vec![0, 16, 10, 22],
        is_light: Some(TileLight {
            light: default_point_light(),
            local_offset: Vec3::new(0., 0., 0.5),
        }),
        ..Default::default()
    });
}
