use std::collections::HashMap;

use bevy::prelude::{Entity, Transform};

use super::{health::DamageModel, inventory::SlotType};

pub struct InventoryItem {
    pub in_inventory_of_entity : Option<Entity>,
    pub attachment_transforms : HashMap<String, Transform>,
    pub drop_transform : Transform,
    pub slot_type : SlotType,
    pub is_attached_when_worn : bool,
    pub combat_animation: CombatAnimation,
    pub combat_type: CombatType,
    pub combat_damage_model : DamageModel,
    pub combat_sound_set : MeleeCombatSoundSet,
}

pub enum CombatAnimation {
    OneHandedMeleePunch
}

pub enum CombatType {
    MeleeDirect,
}

pub struct MeleeCombatSoundSet {
    pub miss : Vec<String>,
    pub hit_soft : Vec<String>,
    pub hit_blocked : Vec<String>,
}

/*
impl MeleeCombatSoundSet {

    pub fn spawn_miss_sfx() {

    }

    pub fn spawn_hit_sfx() {

    }

    pub fn spawn_hit_blocked() {

    }
}
*/

impl Default for MeleeCombatSoundSet {
    fn default() -> Self {
        Self {
            miss: vec![
                "swing1".to_string(),
                "swing2".to_string(),
                "swing3".to_string(),
                "swing4".to_string(),
            ],
            hit_soft: vec![
                "punch1".to_string(),
                "punch2".to_string(),
                "punch3".to_string(),
                "punch4".to_string(),
            ],
            hit_blocked: vec![
                "block1".to_string(),
                "block2".to_string(),
                "block3".to_string(),
            ],
        }
    }
}

#[derive(Clone)]
pub enum HitSoundSurface {
    Soft,
    Metaloid,
}
