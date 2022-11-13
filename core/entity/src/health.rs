use std::collections::HashMap;

use bevy::prelude::Component;

/// The data for entities and gridmap cells that have health.
#[derive(Clone)]
#[cfg(feature = "server")]
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
#[cfg(feature = "server")]
pub enum HitSoundSurface {
    Soft,
    Metaloid,
}

/// All potential damage flags.
#[allow(dead_code)]
#[derive(PartialEq, Clone)]
#[cfg(feature = "server")]
pub enum DamageFlag {
    SoftDamage, //Ie fists.
    WeakLethalLaser,
    Stun(f32),
    Floor(f32),
}
/// Health flags acting as damage amplifiers or negators. Such as the armour plating flag.
#[allow(dead_code)]
#[derive(PartialEq, Clone)]
#[cfg(feature = "server")]
pub enum HealthFlag {
    ArmourPlated,
    HeadBruteDefence(f32),
    TorsoBruteDefence(f32),
}

/// The health component as a container.
#[derive(Component)]
#[cfg(feature = "server")]
pub struct HealthComponent {
    pub health: Health,
}

#[cfg(feature = "server")]
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
#[cfg(feature = "server")]
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
#[cfg(feature = "server")]
pub enum HealthContainer {
    Humanoid(HumanoidHealth),
    Entity(EntityContainer),
    Structure(StructureHealth),
}
/// Health data for structures like gridmap cells.
#[derive(Clone, Default)]
#[cfg(feature = "server")]
pub struct StructureHealth {
    pub brute: f32,
    pub burn: f32,
    pub toxin: f32,
}

/// The health data for entities.
#[derive(Default, Clone)]
#[cfg(feature = "server")]
pub struct EntityContainer {
    pub brute: f32,
    pub burn: f32,
    pub toxin: f32,
}
