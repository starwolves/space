use bevy::prelude::{Res, ResMut, Transform};
use entity::examine::RichName;
use resources::is_server::is_server;

use crate::{
    grid::{CellType, CellTypeName, TileProperties},
    init::InitTileProperties,
};

use std::collections::HashMap;

use resources::{grid::CellFace, math::Vec3Int};

use crate::grid::{FullCell, Gridmap, GroupTypeId, GroupTypeName};

use super::generic_assets::GenericMeshes;

pub(crate) fn init_generic_wall(mut init: ResMut<InitTileProperties>, meshes: Res<GenericMeshes>) {
    let mut default_isometry = Transform::IDENTITY;

    default_isometry.translation.y = -0.5;

    let mesh_option;
    if !is_server() {
        mesh_option = Some(meshes.wall.clone());
    } else {
        mesh_option = None;
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
        ..Default::default()
    });
}
pub(crate) fn init_generic_wall_group(mut gridmap_data: ResMut<Gridmap>) {
    let mut wall_group = HashMap::new();
    let group_id = GroupTypeId(gridmap_data.group_type_incremental);
    gridmap_data.group_type_incremental += 1;
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
    gridmap_data.groups.insert(group_id, wall_group);
    gridmap_data
        .group_id_map
        .insert(GroupTypeName("generic_wall_group".to_string()), group_id);
    gridmap_data
        .id_group_map
        .insert(group_id, GroupTypeName("generic_wall_group".to_string()));
}
