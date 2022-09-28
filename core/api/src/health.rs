use std::collections::HashMap;

use bevy::prelude::{Component, Entity};
use networking_macros::NetMessage;

use crate::{
    combat::{DamageFlag, HitResult},
    gridmap::CellData,
    network::{PendingMessage, PendingNetworkMessage, ReliableServerMessage},
};

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

/// The health component as a container.
#[derive(Component)]
pub struct HealthComponent {
    pub health: Health,
}

/// For sound effects.
#[derive(Clone)]
pub enum HitSoundSurface {
    Soft,
    Metaloid,
}

/// Health flags acting as damage amplifiers or negators. Such as the armour plating flag.
#[allow(dead_code)]
#[derive(PartialEq, Clone)]
pub enum HealthFlag {
    ArmourPlated,
    HeadBruteDefence(f32),
    TorsoBruteDefence(f32),
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

/// A pending cell update like a cell construction.
pub struct CellUpdate {
    pub entities_received: Vec<Entity>,
    pub cell_data: CellData,
}

/// Health data for structures like gridmap cells.
#[derive(Clone, Default)]
pub struct StructureHealth {
    pub brute: f32,
    pub burn: f32,
    pub toxin: f32,
}

/// General function for returning the results of damage application.
pub fn calculate_damage(
    health_flags: &HashMap<u32, HealthFlag>,
    damage_flags: &HashMap<u32, DamageFlag>,

    brute: &f32,
    burn: &f32,
    toxin: &f32,
) -> (f32, f32, f32, HitResult) {
    let mut output_brute = brute.clone();
    let mut output_burn = burn.clone();
    let output_toxin = toxin.clone();

    let mut hit_result = HitResult::HitSoft;

    let mut damager_flags = vec![];

    for damage_flag in damage_flags.values() {
        damager_flags.push(damage_flag);
    }

    let mut structure_health_flags = vec![];

    for stucture_health_flag in health_flags.values() {
        structure_health_flags.push(stucture_health_flag);
    }

    let is_armour_plated = structure_health_flags.contains(&&HealthFlag::ArmourPlated);

    if damager_flags.contains(&&DamageFlag::SoftDamage) && is_armour_plated {
        output_brute = 0.;
        hit_result = HitResult::Blocked;
    } else if damager_flags.contains(&&DamageFlag::WeakLethalLaser) && is_armour_plated {
        output_burn *= 0.05;
        hit_result = HitResult::Blocked;
    }

    (output_brute, output_burn, output_toxin, hit_result)
}

/// The health data for entities.
#[derive(Default, Clone)]
pub struct EntityContainer {
    pub brute: f32,
    pub burn: f32,
    pub toxin: f32,
}
#[derive(NetMessage)]
pub struct NetHealthUpdate {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
#[derive(NetMessage)]
pub struct NetHealth {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
