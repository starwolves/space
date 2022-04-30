use bevy_ecs::{entity::Entity, system::Query};
use bevy_math::Vec3;
use bevy_rapier3d::prelude::RigidBodyPositionComponent;

use crate::core::{
    connected_player::components::ConnectedPlayer,
    gridmap::{functions::gridmap_functions::cell_id_to_world, resources::Vec3Int},
};

pub fn debug_player(
    connected_players: Query<(Entity, &ConnectedPlayer, &RigidBodyPositionComponent)>,
) {
    for (_, _, rigid_body_position) in connected_players.iter() {
        let mut player_location: Vec3 = rigid_body_position.position.translation.into();
        player_location.y = -1.;
        println!(
            "{:?}",
            player_location.distance(cell_id_to_world(Vec3Int {
                x: 11,
                y: -1,
                z: 13
            })),
        );
    }
}
