use doryen_fov::FovRecursiveShadowCasting;

use crate::space_core::resources::precalculated_fov_data::Vec2Int;

pub struct Senser {
    pub cell_id : Vec2Int,
    pub fov : FovRecursiveShadowCasting,
}

pub const FOV_MAP_WIDTH : usize = 2000;
pub const FOV_MAP_HEIGHT : usize = 2000;
