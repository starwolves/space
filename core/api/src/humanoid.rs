use std::collections::HashMap;

use bevy::prelude::Entity;

/// How far melee fists attacks can reach.
pub const MELEE_FISTS_REACH: f32 = 1.2;
/// Resource keeping track of which in-game character names are taken.
#[derive(Default, Clone)]
pub struct UsedNames {
    /// Character names.
    pub names: HashMap<String, Entity>,
    /// Global user names.
    pub user_names: HashMap<String, Entity>,
    pub player_i: u32,
    pub dummy_i: u32,
}
