use bevy::prelude::{AssetServer, Assets, Res, ResMut, StandardMaterial, Transform};
use bevy_xpbd_3d::prelude::Collider;
use entity::examine::RichName;
use resources::modes::{is_server, AppMode};

use crate::{
    grid::{CellType, CellTypeName, TileProperties},
    init::InitTileProperties,
};

use super::{generic_assets::GenericMeshes, wall_flat::WallMaterials};
pub(crate) fn init_vents_access_material(
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut res: ResMut<WallMaterials>,
) {
    let albedo_texture_handle = asset_server.load("gridmap/vents_access/vents_base.png");
    let metallic_roughness_texture_handle =
        asset_server.load("gridmap/vents_access/vents_metal_rough.png");

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(albedo_texture_handle),
        metallic_roughness_texture: Some(metallic_roughness_texture_handle),
        perceptual_roughness: 0.9,
        metallic: 0.97,
        ..Default::default()
    });
    res.vents = material_handle;
}

pub(crate) fn init_vents_access(
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
        mesh_option = Some(meshes.vents_access.clone_weak());

        material_option = Some(mat.vents.clone_weak());
    } else {
        mesh_option = None;
        material_option = None;
    }
    init.properties.push(TileProperties {
        name_id: CellTypeName("vents_access".to_string()),
        name: RichName {
            name: "vent entry point".to_string(),
            n: true,
            the: false,
        },
        description: "It looks breakable.".to_string(),
        constructable: true,
        mesh_option,
        cell_type: CellType::Wall,
        material_option,
        collider: Collider::cuboid(1., 1., 0.2),
        //x_rotations: vec![0, 16, 10, 22],
        ..Default::default()
    });
}
