use std::collections::HashMap;

#[derive(Component, Clone)]
pub struct Health {
    pub health_container: HealthContainer,
    pub health_flags: HashMap<u32, HealthFlag>,
    pub damage_flags: HashMap<u32, DamageFlag>,
    pub hit_sound_surface: HitSoundSurface,
    pub is_combat_obstacle: bool,
    pub is_laser_obstacle: bool,
    pub is_reach_obstacle: bool,
}
#[derive(Component)]
pub struct HealthComponent {
    pub health: Health,
}

#[derive(Clone)]
pub enum HitSoundSurface {
    Soft,
    Metaloid,
}

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

#[derive(Clone)]
pub enum HealthContainer {
    Humanoid(HumanoidHealth),
    Entity(EntityContainer),
    Structure(StructureHealth),
}

pub struct CellUpdate {
    pub entities_received: Vec<Entity>,
    pub cell_data: CellData,
}

#[derive(Clone, Default)]
pub struct StructureHealth {
    pub brute: f32,
    pub burn: f32,
    pub toxin: f32,
}

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

#[derive(Default, Clone)]
pub struct EntityContainer {
    pub brute: f32,
    pub burn: f32,
    pub toxin: f32,
}

use bevy::prelude::{Component, Entity};

use crate::{
    combat::{DamageFlag, HitResult},
    gridmap::CellData,
    network::{PendingMessage, PendingNetworkMessage, ReliableServerMessage},
};

pub struct NetHealthUpdate {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
impl PendingMessage for NetHealthUpdate {
    fn get_message(&self) -> PendingNetworkMessage {
        PendingNetworkMessage {
            handle: self.handle,
            message: self.message.clone(),
        }
    }
}
pub struct NetHealth {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
impl PendingMessage for NetHealth {
    fn get_message(&self) -> PendingNetworkMessage {
        PendingNetworkMessage {
            handle: self.handle,
            message: self.message.clone(),
        }
    }
}
