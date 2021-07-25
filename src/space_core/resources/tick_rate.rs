use bevy::prelude::{FromWorld, World};

pub struct TickRate {
    pub rate : u8
}


impl FromWorld for TickRate {
    fn from_world(_world: &mut World) -> Self {
        TickRate {
           rate : 24, 
        }
    }
}
