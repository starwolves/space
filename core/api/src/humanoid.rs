use std::collections::HashMap;

use bevy::prelude::Entity;

pub const MELEE_FISTS_REACH: f32 = 1.2;
#[derive(Default, Clone)]
pub struct UsedNames {
    pub names: HashMap<String, Entity>,
    pub user_names: HashMap<String, Entity>,
    pub player_i: u32,
    pub dummy_i: u32,
}
