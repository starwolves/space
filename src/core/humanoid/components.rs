use crate::core::health::components::{DamageFlag, DamageModel};
use crate::core::inventory_item::components::CombatSoundSet;
use bevy_core::Timer;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::Component;
use std::collections::HashMap;
use std::time::Duration;

#[derive(Component)]
pub struct Humanoid {
    pub current_lower_animation_state: CharacterAnimationState,
    pub character_name: String,
    pub combat_mode: bool,
    pub facing_direction: f32,
    pub is_attacking: bool,
    pub next_attack_timer: Timer,
    pub default_melee_damage_model: DamageModel,
    pub default_melee_sound_set: CombatSoundSet,
}

pub enum CharacterAnimationState {
    Idle,
    Jogging,
    Sprinting,
}

#[derive(Component)]
pub struct LinkedFootstepsSprinting {
    pub entity: Entity,
}

#[derive(Component)]
pub struct LinkedFootstepsWalking {
    pub entity: Entity,
}

const FIRST_MELEE_TIME: u64 = 433;

impl Default for Humanoid {
    fn default() -> Self {
        let mut t = Timer::new(Duration::from_millis(FIRST_MELEE_TIME), false);
        let mut first_damage_flags = HashMap::new();
        first_damage_flags.insert(0, DamageFlag::SoftDamage);
        t.tick(Duration::from_millis(FIRST_MELEE_TIME));
        Self {
            current_lower_animation_state: CharacterAnimationState::Idle,
            character_name: "".to_string(),
            combat_mode: false,
            facing_direction: 0.,
            is_attacking: false,
            next_attack_timer: t,
            default_melee_damage_model: DamageModel {
                brute: 5.,
                damage_flags: first_damage_flags,
                ..Default::default()
            },
            default_melee_sound_set: CombatSoundSet::default(),
        }
    }
}
