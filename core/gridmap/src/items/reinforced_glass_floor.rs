use bevy::{
    gltf::GltfMesh,
    prelude::{AlphaMode, AssetServer, Assets, Handle, Res, ResMut, Resource, StandardMaterial},
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
pub struct ReinforcedGlassFloorMaterial {
    pub material_handle: Handle<StandardMaterial>,
}

pub(crate) fn init_reinforced_glass_floor_material(
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut res: ResMut<ReinforcedGlassFloorMaterial>,
) {
    let albedo_texture_handle = asset_server
        .load("gridmap/floor_template/reinforced_glass/reinforced_glass_floor_base.png");
    let metallic_roughness_texture_handle = asset_server
        .load("gridmap/floor_template/reinforced_glass/reinforced_glass_floor_metal_rough.png");

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(albedo_texture_handle.clone()),
        metallic_roughness_texture: Some(metallic_roughness_texture_handle.clone()),
        alpha_mode: AlphaMode::Blend,
        perceptual_roughness: 0.9,
        metallic: 0.97,

        ..Default::default()
    });
    res.material_handle = material_handle;
}

pub(crate) fn init_reinforced_glass_floor(
    mut init: ResMut<InitTileProperties>,
    meshes: Res<GenericMeshes>,
    mat: Res<ReinforcedGlassFloorMaterial>,
    app_mode: Res<AppMode>,
) {
    let mesh_option: Option<Handle<GltfMesh>>;
    let material_option;

    if !is_server() || matches!(*app_mode, AppMode::Correction) {
        mesh_option = Some(meshes.floor.clone());
        material_option = Some(mat.material_handle.clone());
    } else {
        mesh_option = None;
        material_option = None;
    }

    init.properties.push(TileProperties {
        name_id: CellTypeName("reinforced_glass_floor".to_string()),
        name: RichName {
            name: "reinforced glass floor".to_string(),
            n: true,
            the: false,
        },
        description: "A reinforced glass floor tile.".to_string(),
        constructable: true,
        floor_cell: true,
        mesh_option,
        cell_type: CellType::Floor,
        material_option,
        collider: Collider::cuboid(1., 0.2, 1.),
        ..Default::default()
    });
}
