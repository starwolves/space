use std::collections::HashMap;

use bevy_ecs::{
    entity::Entity,
    prelude::{FromWorld, World},
};

pub struct ClientHealthUICache {
    pub cache: HashMap<Entity, ClientHealthUI>,
}

impl FromWorld for ClientHealthUICache {
    fn from_world(_world: &mut World) -> Self {
        ClientHealthUICache {
            cache: HashMap::new(),
        }
    }
}

pub struct ClientHealthUI {
    pub head_damage: UIDamageType,
    pub torso_damage: UIDamageType,
    pub left_arm_damage: UIDamageType,
    pub right_arm_damage: UIDamageType,
    pub left_leg_damage: UIDamageType,
    pub right_leg_damage: UIDamageType,
}

pub enum UIDamageType {
    None,
    Light,
    Moderate,
    Heavy,
}
