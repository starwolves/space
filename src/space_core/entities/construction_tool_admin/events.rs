use bevy::prelude::Entity;

use crate::space_core::generics::networking::resources::{GridMapType, ReliableServerMessage};


pub struct InputConstruct {
    pub handle : u32,
    pub target_cell : (GridMapType, i16,i16,i16),
    pub belonging_entity : u64,
}

pub struct InputConstructionOptionsSelection {
    pub handle : u32,
    pub menu_selection : String,
    // Entity has been validated.
    pub entity : Entity,
}

pub struct InputConstructionOptions {
    pub handle : u32,
    pub belonging_entity : u64,
}

pub struct InputDeconstruct {
    pub handle : u32,
    pub target_cell : (GridMapType, i16,i16,i16),
    pub belonging_entity : u64,
}

pub struct NetConstructionTool {
    pub handle : u32,
    pub message : ReliableServerMessage
}
