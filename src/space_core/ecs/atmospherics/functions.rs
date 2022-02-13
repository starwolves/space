use crate::space_core::ecs::gridmap::resources::{Vec2Int, FOV_MAP_WIDTH};

pub fn get_atmos_index(
    id : Vec2Int,
) -> usize {

    let idx : u32 = (id.x + (FOV_MAP_WIDTH / 2) as i16) as u32;
    let idy : u32 = (id.y + (FOV_MAP_WIDTH / 2) as i16) as u32;

    (idx + (idy*FOV_MAP_WIDTH as u32)) as usize

}

pub fn get_atmos_id(
    i : usize
) -> Vec2Int {

    let y = (i as f32 /FOV_MAP_WIDTH as f32).floor() as usize;
    let x = i - (y*FOV_MAP_WIDTH);

    Vec2Int {
        x: x as i16 - (FOV_MAP_WIDTH as i16 / 2),
        y: y as i16 - (FOV_MAP_WIDTH as i16 / 2),
    }

}
