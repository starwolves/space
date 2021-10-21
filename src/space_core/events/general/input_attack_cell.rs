use bevy::prelude::Entity;

use crate::space_core::resources::doryen_fov::Vec3Int;

pub struct InputAttackCell {
    pub handle : u32,
    pub entity : Entity,
    pub id : Vec3Int,
}
