use bevy::prelude::{FromWorld, World, warn};
use doryen_fov::{MapData};

use serde::{Serialize, Deserialize};

pub struct DoryenMap {

    pub map : MapData,

}

impl FromWorld for DoryenMap {
    fn from_world(_world: &mut World) -> Self {

        DoryenMap {
            
            map : MapData::new(FOV_MAP_WIDTH, FOV_MAP_HEIGHT),

        }
    }
}

pub fn to_doryen_coordinates(x : i16, y : i16) -> (usize, usize){

    let mut n_x=x+FOV_MAP_WIDTH as i16/2;
    let mut n_y=y+FOV_MAP_HEIGHT as i16/2;

    if doryen_coordinates_out_of_range(n_x as usize, n_y as usize) {
        warn!("Out of bounds x:{} y:{}",x,y);
        n_x=0;
        n_y=0;
    }

    (n_x as usize,n_y as usize)

}


pub fn doryen_coordinates_out_of_range(x : usize, y : usize) -> bool {
    x > FOV_MAP_WIDTH || y > FOV_MAP_HEIGHT
}


#[derive(PartialEq,Eq, Hash, Copy, Clone, Debug)]
pub struct Vec2Int {
    pub x : i16,
    pub y : i16,   
}
#[derive(PartialEq,Eq, Hash, Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Vec3Int {
    pub x : i16,
    pub y : i16,  
    pub z : i16,  
}


// Turning up these values drastically increases fov calculation time.
// The largest maps we can support with f32 accuracy is a 2000x2000 tiled map.
// FOV calculation time will take 10x-15x slower, up to 2-3ms for just a single player calculation.
// For bigger maps than 500x500 gridmaps we need a new and better FOV algorithm.
pub const FOV_MAP_WIDTH : usize = 500;
pub const FOV_MAP_HEIGHT : usize = 500;
