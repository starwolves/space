use bevy::{math::Quat, prelude::{FromWorld, Transform, World}};

use serde::{Deserialize};

use crate::space_core::functions::converters::string_to_type_converters::string_transform_to_transform;



#[derive(Deserialize)]
pub struct SpawnPointRaw {
    pub point_type : String,
    pub transform : String
}

pub struct SpawnPoint {
    pub point_type : String,
    pub transform : Transform
}

pub struct SpawnPoints {
    pub list : Vec<SpawnPoint>,
    pub i : usize
}

impl SpawnPoint {
    pub fn new(raw : &SpawnPointRaw) -> SpawnPoint {

        let mut this_transform = string_transform_to_transform(&raw.transform);
        
        this_transform.translation.y = 0.05;

        this_transform.rotation = Quat::IDENTITY;


        SpawnPoint {
            point_type : raw.point_type.clone(),
            transform : this_transform
        }


    }
}

impl FromWorld for SpawnPoints {
    fn from_world(_world: &mut World) -> Self {
        SpawnPoints {
            list : vec![],
            i : 0,
        }
    }
}
