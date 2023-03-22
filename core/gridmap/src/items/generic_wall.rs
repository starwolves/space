use bevy::prelude::{AssetServer, Res, ResMut, Transform};
use entity::examine::RichName;
use resources::is_server::is_server;

use crate::{
    grid::{CellType, CellTypeName, Gridmap, TileProperties},
    init::InitTileProperties,
};
pub(crate) fn init_wall_properties(
    gridmap_data: Res<Gridmap>,
    assets: Res<AssetServer>,
    mut init: ResMut<InitTileProperties>,
) {
    let mut default_isometry = Transform::IDENTITY;

    default_isometry.translation.y = -0.5;

    let mesh_option;
    if !is_server() {
        mesh_option = Some(assets.load("models/wall/wall.glb#Scene0"));
    } else {
        mesh_option = None;
    }
    let id = *gridmap_data
        .main_name_id_map
        .get(&CellTypeName("generic_wall_1".to_owned()))
        .unwrap();
    init.properties.push(TileProperties {
        id: id,
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
