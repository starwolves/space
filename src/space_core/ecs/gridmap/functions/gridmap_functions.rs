use bevy::math::Vec3;

use crate::space_core::ecs::gridmap::resources::Vec3Int;


const CELL_SIZE : f32 = 2.;
const Y_CENTER_OFFSET : f32 = 1.;

pub fn cell_id_to_world(cell_id : Vec3Int) -> Vec3 {

    let mut world_position : Vec3 = Vec3::ZERO;

    world_position.x = (cell_id.x as f32 * CELL_SIZE) + Y_CENTER_OFFSET;
    world_position.y = (cell_id.y as f32 * CELL_SIZE) + Y_CENTER_OFFSET;
    world_position.z = (cell_id.z as f32 * CELL_SIZE) + Y_CENTER_OFFSET;

    world_position

}

pub fn world_to_cell_id(position : Vec3) -> Vec3Int {
    
    let map_pos = position / CELL_SIZE;

    Vec3Int {
        x:map_pos.x.floor() as i16,
        y: map_pos.y.floor() as i16,
        z: map_pos.z.floor() as i16,
    }

}
