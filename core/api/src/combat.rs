use std::collections::HashMap;

use bevy::math::Vec3;
use serde::{Deserialize, Serialize};

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

#[derive(Clone)]
pub struct DamageModel {
    pub brute: f32,
    pub burn: f32,
    pub toxin: f32,
    pub damage_flags: HashMap<u32, DamageFlag>,
}

impl Default for DamageModel {
    fn default() -> Self {
        Self {
            brute: 0.,
            burn: 0.,
            toxin: 0.,
            damage_flags: HashMap::new(),
        }
    }
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
    Projectile(ProjectileType),
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum ProjectileType {
    Laser((f32, f32, f32, f32), f32, f32, f32),
    Ballistic,
}
