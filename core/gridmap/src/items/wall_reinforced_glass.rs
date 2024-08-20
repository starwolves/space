use bevy::prelude::{AlphaMode, AssetServer, Assets, Res, ResMut, StandardMaterial, Transform};
use bevy_xpbd_3d::prelude::Collider;
use entity::examine::RichName;
use resources::modes::{is_server, AppMode};

use crate::{
    grid::{CellType, CellTypeName, TileProperties},
    init::InitTileProperties,
};

use super::{generic_assets::GenericMeshes, wall_flat::WallMaterials};

pub(crate) fn init_wall_reinforced_glass_material(
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut res: ResMut<WallMaterials>,
) {
    let albedo_texture_handle =
        asset_server.load("gridmap/wall_reinforced/glass/wall_reinforced_base.png");
    let metallic_roughness_texture_handle =
        asset_server.load("gridmap/wall_reinforced/glass/wall_reinforced_metal_rough.png");

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(albedo_texture_handle),
        metallic_roughness_texture: Some(metallic_roughness_texture_handle),
        perceptual_roughness: 0.9,
        metallic: 0.97,
        alpha_mode: AlphaMode::Blend,
        thickness: 0.2,
        ior: 1.52,
        diffuse_transmission: 1.,
        ..Default::default()
    });
    res.wall_reinforced_glass = material_handle;
}

pub(crate) fn init_wall_reinforced_glass(
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
        mesh_option = Some(meshes.wall_reinforced.clone_weak());

        material_option = Some(mat.wall_reinforced_glass.clone_weak());
    } else {
        mesh_option = None;
        material_option = None;
    }
    init.properties.push(TileProperties {
        name_id: CellTypeName("wall_reinforced_glass".to_string()),
        name: RichName {
            name: "reinforced glass wall".to_string(),
            n: true,
            the: false,
        },
        description: "A reinforced glass wall.".to_string(),
        constructable: true,
        mesh_option,
        cell_type: CellType::Wall,
        material_option,
        collider: Collider::cuboid(1., 1., 0.2),
        ..Default::default()
    });
}
