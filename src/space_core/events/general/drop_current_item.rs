use bevy::{math::Vec3, prelude::Entity};

pub struct InputDropCurrentItem {
    pub handle : u32,
    pub pickuper_entity : Entity,
    pub input_position_option : Option<Vec3>,
}
