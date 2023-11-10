use bevy::prelude::{
    AssetServer, Assets, Handle, Res, ResMut, Resource, StandardMaterial, Transform,
};
use bevy_xpbd_3d::prelude::Collider;
use entity::examine::RichName;
use resources::modes::{is_server, Mode};

use crate::{
    grid::{CellType, CellTypeName, TileGroup, TileProperties},
    init::{InitTileGroups, InitTileProperties},
};

use std::collections::HashMap;

use resources::{grid::CellFace, math::Vec3Int};

use crate::grid::{FullCell, Gridmap, GroupTypeName};

use super::generic_assets::GenericMeshes;

#[derive(Default, Resource)]
pub struct BridgeWallMaterial {
    pub material_handle: Handle<StandardMaterial>,
}

pub(crate) fn init_bridge_wall_material(
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut res: ResMut<BridgeWallMaterial>,
) {
    let albedo_texture_handle = asset_server.load("gridmap/wall_template/bridge/bridge_base.png");
    let metallic_roughness_texture_handle =
        asset_server.load("gridmap/wall_template/bridge/bridge_metal_rough.png");

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(albedo_texture_handle.clone()),
        metallic_roughness_texture: Some(metallic_roughness_texture_handle.clone()),
        perceptual_roughness: 0.9,
        metallic: 0.97,
        ..Default::default()
    });
    res.material_handle = material_handle;
}

pub(crate) fn init_bridge_wall(
    mut init: ResMut<InitTileProperties>,
    meshes: Res<GenericMeshes>,
    mat: Res<BridgeWallMaterial>,
    app_mode: Res<Mode>,
) {
    let mut default_isometry = Transform::IDENTITY;

    default_isometry.translation.y = -0.5;

    let mesh_option;
    let material_option;
    if !is_server() || matches!(*app_mode, Mode::Correction) {
        mesh_option = Some(meshes.wall.clone());

        material_option = Some(mat.material_handle.clone());
    } else {
        mesh_option = None;
        material_option = None;
    }
    init.properties.push(TileProperties {
        name_id: CellTypeName("bridge_wall".to_string()),
        name: RichName {
            name: "aluminum bridge wall".to_string(),
            n: true,
            the: false,
        },
        description: "A bridge wall tile.".to_string(),
        constructable: true,
        mesh_option,
        cell_type: CellType::Wall,
        material_option,
        collider: Collider::cuboid(1., 1., 0.2),
        ..Default::default()
    });
}
pub(crate) fn init_bridge_wall_group(
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
                .get(&CellTypeName("bridge_wall".to_string()))
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
                .get(&CellTypeName("bridge_wall".to_string()))
                .unwrap(),
            entity_option: None,
        },
    );

    groups.groups.push(TileGroup {
        name_id: GroupTypeName("bridge_wall_group".to_string()),
        map: wall_group,
    });
}
