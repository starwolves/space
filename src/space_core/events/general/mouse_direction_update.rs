use bevy::{ prelude::Entity};

pub struct InputMouseDirectionUpdate {
    pub handle : u32,
    pub entity : Entity,
    pub direction : f32,
    pub time_stamp : u64,
}
