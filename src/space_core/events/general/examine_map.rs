use bevy::prelude::Entity;

use crate::space_core::{resources::{doryen_fov::Vec3Int, network_messages::GridMapType}};

pub struct ExamineMap{
    pub handle : u32,
    pub entity : Entity,
    pub gridmap_type : GridMapType,
    pub gridmap_cell_id : Vec3Int,
}
