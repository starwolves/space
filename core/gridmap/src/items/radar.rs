use bevy::{
    asset::Handle,
    ecs::system::Resource,
    prelude::{AssetServer, Assets, Res, ResMut, StandardMaterial, Transform},
};
use bevy_xpbd_3d::prelude::Collider;
use entity::examine::RichName;
use resources::modes::{is_server, AppMode};

use super::generic_assets::GenericMeshes;
use crate::{
    grid::{CellType, CellTypeName, TileProperties},
    init::InitTileProperties,
};

#[derive(Default, Resource)]
pub struct RadarMaterials {
    pub medium: Handle<StandardMaterial>,
}

pub(crate) fn init_radar_material(
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut res: ResMut<RadarMaterials>,
) {
    let albedo_texture_handle = asset_server.load("gridmap/radar/radar_base.png");
    let metallic_roughness_texture_handle =
        asset_server.load("gridmap/radar/radar_metal_rough.png");

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(albedo_texture_handle),
        metallic_roughness_texture: Some(metallic_roughness_texture_handle),
        perceptual_roughness: 0.9,
        metallic: 0.97,
        ..Default::default()
    });
    res.medium = material_handle;
}

pub(crate) fn init_radar(
    mut init: ResMut<InitTileProperties>,
    meshes: Res<GenericMeshes>,
    mat: Res<RadarMaterials>,
    app_mode: Res<AppMode>,
) {
    let mut default_isometry = Transform::IDENTITY;

    default_isometry.translation.y = -0.5;

    let mesh_option;
    let material_option;
    if !is_server() || matches!(*app_mode, AppMode::Correction) {
        mesh_option = Some(meshes.radar.clone_weak());

        material_option = Some(mat.medium.clone_weak());
    } else {
        mesh_option = None;
        material_option = None;
    }
    init.properties.push(TileProperties {
        name_id: CellTypeName("radar_medium".to_string()),
        name: RichName {
            name: "radar".to_string(),
            n: true,
            the: false,
        },
        description: "A radar tile.".to_string(),
        constructable: true,
        mesh_option,
        cell_type: CellType::Center,
        material_option,
        x_rotations: vec![0, 16, 10, 22],
        collider: Collider::cuboid(1., 1., 0.2),
        ..Default::default()
    });
}
