use bevy::prelude::Entity;

use crate::space_core::{resources::{network_messages::GridMapType, precalculated_fov_data::Vec3Int}};

pub struct ExamineMap{
    pub handle : u32,
    pub entity : Entity,
    pub gridmap_type : GridMapType,
    pub gridmap_cell_id : Vec3Int,
}
