use std::collections::HashMap;

use bevy::prelude::{Component, Entity, Transform};

use crate::core::{
    combat::attack::{CombatAttackAnimation, CombatSoundSet, CombatStandardAnimation, CombatType},
    health::health::DamageModel,
    inventory::inventory::SlotType,
    tab_actions::tab_action::TabAction,
};

#[derive(Component)]
pub struct InventoryItem {
    pub in_inventory_of_entity: Option<Entity>,
    pub attachment_transforms: HashMap<String, Transform>,
    pub drop_transform: Transform,
    pub slot_type: SlotType,
    pub is_attached_when_worn: bool,
    pub combat_standard_animation: CombatStandardAnimation,
    pub combat_attack_animation: CombatAttackAnimation,
    pub combat_type: CombatType,
    pub combat_melee_damage_model: DamageModel,
    pub combat_projectile_damage_model: Option<DamageModel>,
    pub combat_melee_sound_set: CombatSoundSet,
    pub combat_projectile_sound_set: Option<CombatSoundSet>,
    pub combat_melee_text_set: Vec<String>,
    pub combat_projectile_text_set: Option<Vec<String>>,
    pub trigger_melee_text_set: Vec<String>,
    pub trigger_projectile_text_set: Option<Vec<String>>,
    pub active_slot_tab_actions: Vec<TabAction>,
    pub throw_force_factor: f32,
}

impl Default for InventoryItem {
    fn default() -> Self {
        Self {
            in_inventory_of_entity: None,
            attachment_transforms: HashMap::default(),
            drop_transform: Transform::default(),
            slot_type: SlotType::Generic,
            is_attached_when_worn: true,
            combat_standard_animation: CombatStandardAnimation::StandardStance,
            combat_attack_animation: CombatAttackAnimation::OneHandedMeleePunch,
            combat_type: CombatType::MeleeDirect,
            combat_melee_damage_model: DamageModel::default(),
            combat_projectile_damage_model: None,
            combat_melee_sound_set: CombatSoundSet::default(),
            combat_projectile_sound_set: None,
            combat_melee_text_set: InventoryItem::get_default_strike_words(),
            combat_projectile_text_set: None,
            trigger_melee_text_set: InventoryItem::get_default_trigger_melee_words(),
            trigger_projectile_text_set: None,
            active_slot_tab_actions: vec![],
            throw_force_factor: 1.,
        }
    }
}

impl InventoryItem {
    pub fn get_default_strike_words() -> Vec<String> {
        vec![
            "hit".to_string(),
            "struck".to_string(),
            "striked".to_string(),
        ]
    }
    pub fn get_default_laser_words() -> Vec<String> {
        vec!["shot".to_string(), "hit".to_string(), "beamed".to_string()]
    }
    pub fn get_default_trigger_weapon_words() -> Vec<String> {
        vec!["fired".to_string(), "shot".to_string()]
    }
    pub fn get_default_trigger_fists_words() -> Vec<String> {
        vec!["swung".to_string(), "thrown".to_string()]
    }
    pub fn get_default_trigger_melee_words() -> Vec<String> {
        vec!["swung".to_string()]
    }
    pub fn get_default_fists_words() -> Vec<String> {
        vec![
            "punched".to_string(),
            "hit".to_string(),
            "swung at".to_string(),
        ]
    }
}
