use crate::space_core::ecs::gridmap::resources::{Vec2Int, FOV_MAP_WIDTH};

pub fn get_atmos_index(
    id : Vec2Int,
) -> usize {

    let idx : u32 = (id.x + (FOV_MAP_WIDTH / 2) as i16) as u32;
    let idy : u32 = (id.y + (FOV_MAP_WIDTH / 2) as i16) as u32;

    (idx + (idy*FOV_MAP_WIDTH as u32)) as usize

}
