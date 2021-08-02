use bevy::prelude::Entity;
use doryen_fov::FovRecursiveShadowCasting;

use crate::space_core::resources::doryen_fov::Vec2Int;


pub struct Senser {
    pub cell_id : Vec2Int,
    pub fov : FovRecursiveShadowCasting,
    pub sensing : Vec<Entity>,
}

impl Default for Senser {
    fn default() -> Self {
        Self {
            cell_id: Vec2Int{
                x: 0,
                y: 0
            },
            fov: FovRecursiveShadowCasting::new(FOV_MAP_WIDTH, FOV_MAP_HEIGHT),
            sensing: vec![],
        }
    }
}

// Turning up these values drastically increases fov calculation time.
// The largest maps we can support with f32 accuracy is a 2000x2000 tiled map.
// FOV calculation time will take 10x-15x slower, up to 2-3ms for just a single player calculation.
// For bigger maps than 500x500 gridmaps we need a new and better FOV algorithm.
pub const FOV_MAP_WIDTH : usize = 500;
pub const FOV_MAP_HEIGHT : usize = 500;
