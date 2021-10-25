use bevy::prelude::Entity;

use crate::space_core::resources::{network_messages::GridMapType};


pub struct InputTabAction {
    pub handle : u32,
    pub tab_id : String,
    pub player_entity : Entity,
    pub target_entity_option: Option<u64>,
    pub target_cell_option: Option<(GridMapType, i16,i16,i16)>,
}
