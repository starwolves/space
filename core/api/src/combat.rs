use std::collections::HashMap;

use bevy::{math::Vec3, prelude::Component};
use serde::{Deserialize, Serialize};

use crate::{chat::Color, humanoid::MELEE_FISTS_REACH};

pub struct ProjectileFOV {
    pub laser_projectile: NetProjectileType,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum NetProjectileType {
    Laser((f32, f32, f32, f32), f32, f32, Vec3, Vec3),
    Ballistic,
}

#[allow(dead_code)]
#[derive(PartialEq, Clone)]
pub enum DamageFlag {
    SoftDamage, //Ie fists.
    WeakLethalLaser,
    Stun(f32),
    Floor(f32),
}

#[derive(Clone, Default)]
pub struct DamageModel {
    pub brute: f32,
    pub burn: f32,
    pub toxin: f32,
    pub damage_flags: HashMap<u32, DamageFlag>,
}

pub enum DamageType {
    Melee,
    Projectile,
}

#[allow(dead_code)]
pub enum HitResult {
    HitSoft,
    Blocked,
    Missed,
}
pub enum CombatStandardAnimation {
    StandardStance,
    PistolStance,
}

pub enum CombatAttackAnimation {
    OneHandedMeleePunch,
    PistolShot,
}

#[derive(Clone, Debug)]
pub enum CombatType {
    MeleeDirect,
    Projectile,
}

#[derive(Clone, Debug)]
pub enum ProjectileType {
    Laser((f32, f32, f32, f32), f32, f32, f32),
}

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
            melee_attack_range: MELEE_FISTS_REACH,
            combat_melee_text_set: get_default_strike_words(),
            combat_attack_animation: CombatAttackAnimation::OneHandedMeleePunch,
            trigger_melee_text_set: get_default_trigger_melee_words(),
        }
    }
}

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

pub const DEFAULT_INVENTORY_ITEM_DAMAGE: f32 = 9.;
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
