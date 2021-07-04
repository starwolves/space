use std::collections::HashMap;

use crate::space_core::structs::network_messages::EntityUpdateData;

pub struct EntityUpdates {
    pub updates : HashMap<String,HashMap<String, EntityUpdateData>>,
    pub updates_difference : HashMap<String,HashMap<String, EntityUpdateData>>,
    pub changed_parameters : Vec<String>,
    pub excluded_handles : HashMap<String, Vec<u32>>
}
