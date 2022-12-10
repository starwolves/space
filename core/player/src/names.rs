use bevy::{ecs::entity::Entity, prelude::Resource};
use std::collections::HashMap;
/// Resource keeping track of which in-game character names are taken.
#[derive(Default, Clone, Resource)]
#[cfg(feature = "server")]
pub struct UsedNames {
    /// Character names.
    pub names: HashMap<String, Entity>,
    /// Global user names.
    pub account_name: HashMap<String, Entity>,
    pub player_i: u32,
    pub dummy_i: u32,
}
