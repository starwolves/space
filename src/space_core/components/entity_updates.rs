use std::collections::HashMap;

use crate::space_core::structs::network_messages::EntityUpdateData;

pub struct EntityUpdates {
    pub updates : HashMap<String,HashMap<String, EntityUpdateData>>
}
