use bevy::prelude::{FromWorld, World};

use crate::space_core::ecs::gridmap::{components::Atmospherics, resources::FOV_MAP_WIDTH};

pub struct AtmosphericsResource {
    pub atmospherics : Vec<Atmospherics>,
}

impl FromWorld for AtmosphericsResource {
    fn from_world(_world: &mut World) -> Self {
        AtmosphericsResource {
            atmospherics: vec![Atmospherics::default(); FOV_MAP_WIDTH*FOV_MAP_WIDTH],
        }
    }
}
