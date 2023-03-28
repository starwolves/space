use bevy::prelude::{Res, ResMut, Transform};
use entity::examine::RichName;
use resources::is_server::is_server;

use crate::{
    grid::{CellType, CellTypeName, TileProperties},
    init::InitTileProperties,
};

use super::generic_assets::GenericMeshes;
pub(crate) fn init_wall_properties(
    mut init: ResMut<InitTileProperties>,
    meshes: Res<GenericMeshes>,
) {
    let mut default_isometry = Transform::IDENTITY;

    default_isometry.translation.y = -0.5;

    let mesh_option;
    if !is_server() {
        mesh_option = Some(meshes.wall.clone());
    } else {
        mesh_option = None;
    }
    init.properties.push(TileProperties {
        name_id: CellTypeName("generic_wall_1".to_string()),
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
