use std::collections::HashMap;

use api::health::{EntityContainer, StructureHealth};
use bevy::prelude::Component;

/// The data for entities and gridmap cells that have health.
#[derive(Clone)]
pub struct Health {
    /// The health container.
    pub health_container: HealthContainer,
    /// Health flags like armor.
    pub health_flags: HashMap<u32, HealthFlag>,
    /// Damage flags like stun.
    pub damage_flags: HashMap<u32, DamageFlag>,
    /// For sound hooks.
    pub hit_sound_surface: HitSoundSurface,
    /// Impacts how combat physics queries are performed.
    pub is_combat_obstacle: bool,
    /// Impacts how combat physics queries are performed.
    pub is_laser_obstacle: bool,
    /// Impacts how combat physics queries are performed.
    pub is_reach_obstacle: bool,
}

/// For sound effects.
#[derive(Clone)]
pub enum HitSoundSurface {
    Soft,
    Metaloid,
}

/// All potential damage flags.
#[allow(dead_code)]
#[derive(PartialEq, Clone)]
pub enum DamageFlag {
    SoftDamage, //Ie fists.
    WeakLethalLaser,
    Stun(f32),
    Floor(f32),
}
/// Health flags acting as damage amplifiers or negators. Such as the armour plating flag.
#[allow(dead_code)]
#[derive(PartialEq, Clone)]
pub enum HealthFlag {
    ArmourPlated,
    HeadBruteDefence(f32),
    TorsoBruteDefence(f32),
}

/// The health component as a container.
#[derive(Component)]
pub struct HealthComponent {
    pub health: Health,
}

impl Default for Health {
    fn default() -> Self {
        Self {
            health_container: HealthContainer::Entity(EntityContainer::default()),
            health_flags: HashMap::new(),
            hit_sound_surface: HitSoundSurface::Soft,
            is_combat_obstacle: false,
            is_laser_obstacle: true,
            is_reach_obstacle: false,
            damage_flags: HashMap::new(),
        }
    }
}

/// Health for each limb of a humanoid entity.
#[derive(Debug, Default, Clone)]
pub struct HumanoidHealth {
    pub head_brute: f32,
    pub head_burn: f32,
    pub head_toxin: f32,

    pub torso_brute: f32,
    pub torso_burn: f32,
    pub torso_toxin: f32,

    pub left_arm_brute: f32,
    pub left_arm_burn: f32,
    pub left_arm_toxin: f32,

    pub right_arm_brute: f32,
    pub right_arm_burn: f32,
    pub right_arm_toxin: f32,

    pub right_leg_brute: f32,
    pub right_leg_burn: f32,
    pub right_leg_toxin: f32,

    pub left_leg_brute: f32,
    pub left_leg_burn: f32,
    pub left_leg_toxin: f32,
}

/// Contains health data of the entity.
#[derive(Clone)]
pub enum HealthContainer {
    Humanoid(HumanoidHealth),
    Entity(EntityContainer),
    Structure(StructureHealth),
}
