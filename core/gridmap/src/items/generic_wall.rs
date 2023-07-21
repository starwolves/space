use bevy::prelude::{
    AssetServer, Assets, Handle, Res, ResMut, Resource, StandardMaterial, Transform,
};
use entity::examine::RichName;
use resources::is_server::is_server;

use crate::{
    grid::{CellType, CellTypeName, TileGroup, TileProperties},
    init::{InitTileGroups, InitTileProperties},
};

use std::collections::HashMap;

use resources::{grid::CellFace, math::Vec3Int};

use crate::grid::{FullCell, Gridmap, GroupTypeName};

use super::generic_assets::GenericMeshes;

#[derive(Default, Resource)]
pub struct GenericWallMaterial {
    pub material_handle: Handle<StandardMaterial>,
}

pub(crate) fn init_generic_wall_material(
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut res: ResMut<GenericWallMaterial>,
) {
    let albedo_texture_handle =
        asset_server.load("gridmap/wall_template/generic/generic_wall_base.png");
    let metallic_roughness_texture_handle =
        asset_server.load("gridmap/wall_template/generic/generic_wall_metal_rough.png");

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(albedo_texture_handle.clone()),
        metallic_roughness_texture: Some(metallic_roughness_texture_handle.clone()),
        perceptual_roughness: 0.9,
        metallic: 0.97,
        ..Default::default()
    });
    res.material_handle = material_handle;
}

pub(crate) fn init_generic_wall(
    mut init: ResMut<InitTileProperties>,
    meshes: Res<GenericMeshes>,
    mat: Res<GenericWallMaterial>,
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
        name_id: CellTypeName("generic_wall".to_string()),
        name: RichName {
            name: "aluminum wall".to_string(),
            n: true,
            the: false,
        },
        description: "A generic wall tile.".to_string(),
        constructable: true,
        mesh_option,
        cell_type: CellType::Wall,
        material_option,
        ..Default::default()
    });
}
pub(crate) fn init_generic_wall_group(
    gridmap_data: Res<Gridmap>,
    mut groups: ResMut<InitTileGroups>,
) {
    let mut wall_group = HashMap::new();
    wall_group.insert(
        Vec3Int { x: 0, y: 0, z: 0 },
        FullCell {
            face: CellFace::default(),
            orientation: 0,
            tile_type: *gridmap_data
                .main_name_id_map
                .get(&CellTypeName("generic_wall".to_string()))
                .unwrap(),
            entity_option: None,
        },
    );
    wall_group.insert(
        Vec3Int { x: 0, y: 1, z: 0 },
        FullCell {
            face: CellFace::default(),
            orientation: 0,
            tile_type: *gridmap_data
                .main_name_id_map
                .get(&CellTypeName("generic_wall".to_string()))
                .unwrap(),
            entity_option: None,
        },
    );

    groups.groups.push(TileGroup {
        name_id: GroupTypeName("generic_wall_group".to_string()),
        map: wall_group,
    });
}
