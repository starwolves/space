use bevy::prelude::{
    AssetServer, Assets, Handle, Res, ResMut, Resource, StandardMaterial, Transform,
};
use bevy_xpbd_3d::prelude::Collider;
use entity::examine::RichName;
use resources::modes::{is_server, AppMode};

use crate::{
    grid::{CellType, CellTypeName, TileGroup, TileProperties},
    init::{InitTileGroups, InitTileProperties},
};

use std::collections::HashMap;

use resources::{grid::CellFace, math::Vec3Int};

use crate::grid::{FullCell, Gridmap, GroupTypeName};

use super::generic_assets::GenericMeshes;

#[derive(Default, Resource)]
pub struct WallMaterials {
    pub flat_handle: Handle<StandardMaterial>,
    pub clean_handle: Handle<StandardMaterial>,
    pub exterior_handle: Handle<StandardMaterial>,
    pub low_curb_handle: Handle<StandardMaterial>,
    pub high_curb_handle: Handle<StandardMaterial>,
    pub wall_reinforced_glass: Handle<StandardMaterial>,
    pub evac_clean: Handle<StandardMaterial>,
    pub evac_lights: Handle<StandardMaterial>,
    pub horizontal_light_strip_handle: Handle<StandardMaterial>,
    pub star_lights_handle: Handle<StandardMaterial>,
    pub vents: Handle<StandardMaterial>,
}

pub(crate) fn init_flat_wall_material(
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut res: ResMut<WallMaterials>,
) {
    let albedo_texture_handle =
        asset_server.load("gridmap/wall_flat/generic/generic_wall_base.png");
    let metallic_roughness_texture_handle =
        asset_server.load("gridmap/wall_flat/generic/generic_wall_metal_rough.png");

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(albedo_texture_handle),
        metallic_roughness_texture: Some(metallic_roughness_texture_handle),
        perceptual_roughness: 0.9,
        metallic: 0.97,
        ..Default::default()
    });
    res.flat_handle = material_handle;
}

pub(crate) fn init_flat_wall(
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
        mesh_option = Some(meshes.wall_flat.clone_weak());

        material_option = Some(mat.flat_handle.clone_weak());
    } else {
        mesh_option = None;
        material_option = None;
    }
    init.properties.push(TileProperties {
        name_id: CellTypeName("wall_flat".to_string()),
        name: RichName {
            name: "aluminum wall".to_string(),
            n: true,
            the: false,
        },
        description: "A flat wall tile.".to_string(),
        constructable: true,
        mesh_option,
        cell_type: CellType::Wall,
        material_option,
        collider: Collider::cuboid(1., 1., 0.2),
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
                .name_id_map
                .get(&CellTypeName("wall_flat".to_string()))
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
                .name_id_map
                .get(&CellTypeName("wall_flat".to_string()))
                .unwrap(),
            entity_option: None,
        },
    );

    groups.groups.push(TileGroup {
        name_id: GroupTypeName("wall_flat_group".to_string()),
        map: wall_group,
    });
}
