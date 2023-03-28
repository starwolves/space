use bevy::prelude::{AssetServer, Res, ResMut, Transform};
use entity::examine::RichName;
use resources::is_server::is_server;

use crate::{
    grid::{CellType, CellTypeName, TileProperties},
    init::InitTileProperties,
};
pub(crate) fn init_glass_wall_properties(
    assets: Res<AssetServer>,
    mut init: ResMut<InitTileProperties>,
) {
    let mut default_isometry = Transform::IDENTITY;

    default_isometry.translation.y = -0.5;

    let mesh_option;
    if !is_server() {
        mesh_option = Some(assets.load("models/wall/wall.glb#Mesh0"));
    } else {
        mesh_option = None;
    }
    init.properties.push(TileProperties {
        name_id: CellTypeName("glass_wall_1".to_string()),
        name: RichName {
            name: "glass wall".to_string(),
            n: true,
            the: false,
        },
        description: "Glass.".to_string(),
        constructable: true,
        mesh_option,
        cell_type: CellType::Wall,
        ..Default::default()
    });
}
