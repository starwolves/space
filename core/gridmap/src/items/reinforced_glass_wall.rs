use super::generic_assets::GenericMeshes;
use bevy::prelude::{
    AlphaMode, AssetServer, Assets, Handle, Res, ResMut, Resource, StandardMaterial, Transform,
};
use entity::examine::RichName;
use resources::is_server::is_server;

use crate::{
    grid::{CellType, CellTypeName, TileProperties},
    init::InitTileProperties,
};

#[derive(Default, Resource)]
pub struct ReinforcedGlassWallMaterial {
    pub material_handle: Handle<StandardMaterial>,
}

pub(crate) fn init_reinforced_glass_wall_material(
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut res: ResMut<ReinforcedGlassWallMaterial>,
) {
    let albedo_texture_handle =
        asset_server.load("models/wall_template/reinforced_glass/reinforced_glass_wall_base.png");
    let metallic_roughness_texture_handle = asset_server
        .load("models/wall_template/reinforced_glass/reinforced_glass_wall_metal_rough.png");

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(albedo_texture_handle.clone()),
        metallic_roughness_texture: Some(metallic_roughness_texture_handle.clone()),
        alpha_mode: AlphaMode::Blend,
        ..Default::default()
    });
    res.material_handle = material_handle;
}

pub(crate) fn init_reinforced_glass_wall(
    mut init: ResMut<InitTileProperties>,
    meshes: Res<GenericMeshes>,
    mat: Res<ReinforcedGlassWallMaterial>,
) {
    let mut default_isometry = Transform::IDENTITY;

    default_isometry.translation.y = -0.5;

    let mesh_option;
    let material_option;

    if !is_server() {
        mesh_option = Some(meshes.wall.clone());
        material_option = Some(mat.material_handle.clone());
    } else {
        mesh_option = None;
        material_option = None;
    }
    init.properties.push(TileProperties {
        name_id: CellTypeName("reinforced_glass_wall".to_string()),
        name: RichName {
            name: "reinforced glass wall".to_string(),
            n: false,
            the: false,
        },
        description: "Glass.".to_string(),
        constructable: true,
        mesh_option,
        cell_type: CellType::Wall,
        material_option,
        ..Default::default()
    });
}