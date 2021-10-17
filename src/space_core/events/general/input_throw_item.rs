use bevy::{math::Vec3, prelude::Entity};

pub struct InputThrowItem {
    pub handle : u32,
    pub entity : Entity,
    pub position : Vec3,
    pub angle : f32,
}
