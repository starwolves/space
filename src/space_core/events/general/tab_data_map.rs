use bevy::prelude::Entity;

use crate::space_core::resources::{doryen_fov::Vec3Int, network_messages::GridMapType};

pub struct InputTabDataMap {
    pub handle : u32,
    pub player_entity : Entity,
    pub gridmap_type : GridMapType,
    pub gridmap_cell_id : Vec3Int,
}
