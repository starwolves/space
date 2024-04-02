use bevy::{
    asset::Handle,
    ecs::system::Resource,
    prelude::{AssetServer, Assets, Res, ResMut, StandardMaterial, Transform},
};
use bevy_xpbd_3d::prelude::Collider;
use entity::examine::RichName;
use resources::modes::{is_server, AppMode};

use crate::{
    grid::{CellType, CellTypeName, TileProperties},
    init::InitTileProperties,
};
#[derive(Resource, Default)]
pub struct AirlockMaterials {
    pub evac: Handle<StandardMaterial>,
}

use super::generic_assets::GenericMeshes;
pub(crate) fn init_airlock_evac_material(
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut res: ResMut<AirlockMaterials>,
) {
    let albedo_texture_handle = asset_server.load("gridmap/airlock_evac/airlock_base.png");
    let metallic_roughness_texture_handle =
        asset_server.load("gridmap/airlock_evac/airlock_metal_rough.png");

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(albedo_texture_handle),
        metallic_roughness_texture: Some(metallic_roughness_texture_handle),
        perceptual_roughness: 0.9,
        metallic: 0.97,
        ..Default::default()
    });
    res.evac = material_handle;
}

pub(crate) fn init_airlock_evac(
    mut init: ResMut<InitTileProperties>,
    meshes: Res<GenericMeshes>,
    mat: Res<AirlockMaterials>,
    app_mode: Res<AppMode>,
) {
    let mut default_isometry = Transform::IDENTITY;

    default_isometry.translation.y = -0.5;

    let mesh_option;
    let material_option;
    if !is_server() || matches!(*app_mode, AppMode::Correction) {
        mesh_option = Some(meshes.airlock.clone_weak());

        material_option = Some(mat.evac.clone_weak());
    } else {
        mesh_option = None;
        material_option = None;
    }
    init.properties.push(TileProperties {
        name_id: CellTypeName("airlock_evac".to_string()),
        name: RichName {
            name: "evac airlock".to_string(),
            n: true,
            the: false,
        },
        description: "An evacuation airlock.".to_string(),
        constructable: true,
        mesh_option,
        cell_type: CellType::Wall,
        material_option,
        collider: Collider::cuboid(1., 1., 0.2),
        ..Default::default()
    });
}
