use std::collections::HashMap;

use bevy::prelude::Component;
use entity::health::DamageFlag;
use pawn::pawn::ARMS_REACH;
use text_api::core::Color;
/// The component for items that can be used to perform melee attacks with. Should be used in combination with handlers.
#[derive(Component)]

pub struct MeleeCombat {
    pub combat_melee_damage_model: DamageModel,
    pub melee_attack_range: f32,
    /// The words used for chat hooks.
    pub combat_melee_text_set: Vec<String>,
    pub combat_attack_animation: CombatAttackAnimation,
    /// The words used for chat hooks.
    pub trigger_melee_text_set: Vec<String>,
}

impl Default for MeleeCombat {
    fn default() -> Self {
        Self {
            combat_melee_damage_model: DamageModel::default(),
            melee_attack_range: ARMS_REACH,
            combat_melee_text_set: get_default_strike_words(),
            combat_attack_animation: CombatAttackAnimation::OneHandedMeleePunch,
            trigger_melee_text_set: get_default_trigger_melee_words(),
        }
    }
}

/// The component for items that can be used to perform projectile attacks with. Should be used in combination with handlers.
#[derive(Component)]

pub struct ProjectileCombat {
    pub combat_projectile_damage_model: DamageModel,
    /// For laser project visuals.
    pub laser_color: Color,
    /// For laser project visuals.
    pub laser_height: f32,
    /// For laser project visuals.
    pub laser_radius: f32,
    /// Range of projectile attack.
    pub laser_range: f32,
    /// The words used for chat hooks.
    pub combat_projectile_text_set: Vec<String>,
    /// The words used for chat hooks.
    pub trigger_projectile_text_set: Vec<String>,
}

impl Default for ProjectileCombat {
    fn default() -> Self {
        Self {
            combat_projectile_damage_model: DamageModel::default(),
            laser_color: Color::default(),
            laser_height: 3.,
            laser_radius: 0.025,
            laser_range: 50.,
            combat_projectile_text_set: get_default_laser_words(),
            trigger_projectile_text_set: get_default_trigger_weapon_words(),
        }
    }
}

/// Contains the modularly built damage data of the attack.
#[derive(Clone, Default)]

pub struct DamageModel {
    pub brute: f32,
    pub burn: f32,
    pub toxin: f32,
    pub damage_flags: HashMap<u32, DamageFlag>,
}

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

/// Humanoid animations for combat.

pub enum CombatAttackAnimation {
    OneHandedMeleePunch,
    PistolShot,
}
