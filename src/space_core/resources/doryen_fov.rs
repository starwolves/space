use bevy::prelude::{FromWorld, World};
use doryen_fov::{FovRecursiveShadowCasting, MapData};

const MAP_WIDTH : usize = 500;

pub struct DoryenFOV {

    pub fov : FovRecursiveShadowCasting,

}

pub struct DoryenMap {

    pub map : MapData,

}

impl FromWorld for DoryenFOV {
    fn from_world(_world: &mut World) -> Self {

        DoryenFOV {
            
            fov : FovRecursiveShadowCasting::new(),

        }
    }
}

impl FromWorld for DoryenMap {
    fn from_world(_world: &mut World) -> Self {

        DoryenMap {
            
            map : MapData::new(MAP_WIDTH, MAP_WIDTH),

        }
    }
}

pub fn to_doryen_coordinates(x : i16, y : i16) -> (usize, usize){

    let n_x=x+MAP_WIDTH as i16/2;
    let n_y=y+MAP_WIDTH as i16/2;

    (n_x as usize,n_y as usize)

}
