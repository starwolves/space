use bevy::prelude::Transform;

use serde::{Deserialize};


use crate::space_core::functions::string_to_type_converters::string_transform_to_transform;

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

        SpawnPoint {
            point_type : raw.point_type.clone(),
            transform : string_transform_to_transform(&raw.transform)
        }

    }
}
