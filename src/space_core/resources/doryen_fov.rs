use bevy::prelude::{FromWorld, World};
use doryen_fov::{MapData};

use crate::space_core::components::senser::{FOV_MAP_HEIGHT, FOV_MAP_WIDTH};

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

    let n_x=x+FOV_MAP_WIDTH as i16/2;
    let n_y=y+FOV_MAP_HEIGHT as i16/2;

    (n_x as usize,n_y as usize)

}
