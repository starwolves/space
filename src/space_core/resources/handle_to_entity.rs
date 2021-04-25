use std::collections::HashMap;
use bevy::prelude::Entity;

pub struct HandleToEntity {
    pub map : HashMap<u32, Entity>,
    pub inv_map : HashMap<u32, u32>
}
