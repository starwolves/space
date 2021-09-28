use std::collections::HashMap;

use bevy::prelude::{Entity, FromWorld, World};

pub struct ClientHealthUICache {

    pub cache : HashMap<Entity, ClientHealthUI>

}

impl FromWorld for ClientHealthUICache {
    fn from_world(_world: &mut World) -> Self {

        ClientHealthUICache {
            
            cache : HashMap::new(),

        }
    }
}


pub struct ClientHealthUI {

    pub head_damage : DamageType,
    pub torso_damage : DamageType,
    pub left_arm_damage : DamageType,
    pub right_arm_damage : DamageType,
    pub left_leg_damage : DamageType,
    pub right_leg_damage : DamageType,

}


pub enum DamageType {
    None,
    Light,
    Moderate,
    Heavy,
}
