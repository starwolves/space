use bevy::prelude::{FromWorld, World};

pub struct AuthidI {
    pub i : u16
}

impl FromWorld for AuthidI {
    fn from_world(_world: &mut World) -> Self {
        AuthidI {
           i : 0, 
        }
    }
}
