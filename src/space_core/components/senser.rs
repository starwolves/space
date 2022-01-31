use bevy::prelude::{Entity, Component};
use doryen_fov::FovRecursiveShadowCasting;

use crate::space_core::resources::doryen_fov::{Vec2Int, FOV_MAP_WIDTH, FOV_MAP_HEIGHT};


#[derive(Component)]
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
