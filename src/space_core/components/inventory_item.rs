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
    pub miss : String,
    pub hit_soft : String,
    pub hit_metaloid : String,
}

#[allow(dead_code)]
pub enum HitSoundSurface {
    Soft,
    Metaloid,
}
