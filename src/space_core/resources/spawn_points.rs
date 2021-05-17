use bevy::{math::Quat, prelude::Transform};

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

        let mut this_transform = string_transform_to_transform(&raw.transform);
        this_transform.translation.y = 0.5;

        this_transform.rotation = Quat::from_xyzw(0.,0.,1.,0.);

        SpawnPoint {
            point_type : raw.point_type.clone(),
            transform : this_transform
        }

    }
}
