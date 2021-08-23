use bevy::{math::Vec3, prelude::Entity};

pub struct MouseDirectionUpdate {
    pub handle : u32,
    pub entity : Entity,
    pub direction : Vec3,
}
