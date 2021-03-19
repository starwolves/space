use bevy::math::Vec3;

const CELL_SIZE : f32 = 2.;
const Y_CENTER_OFFSET : f32 = 1.;

pub fn cell_id_to_world(cell_id : Vec3) -> Vec3 {

    let mut world_position : Vec3 = Vec3::zero();

    world_position.x = (cell_id.x * CELL_SIZE) + Y_CENTER_OFFSET;
    world_position.y = (cell_id.y * CELL_SIZE) + Y_CENTER_OFFSET;
    world_position.z = (cell_id.z * CELL_SIZE) + Y_CENTER_OFFSET;

    return world_position;

}
