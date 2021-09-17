use std::time::Duration;

use bevy::core::Timer;

pub struct StandardCharacter {
    pub current_lower_animation_state : CharacterAnimationState,
    pub character_name : String,
    pub combat_mode : bool,
    pub facing_direction : f32,
    pub is_attacking : bool,
    pub next_attack_timer : Timer,
}

pub enum CharacterAnimationState {
    Idle,
    Jogging,
    Sprinting,
}

const FIRST_MELEE_TIME : u64 = 433;

impl Default for StandardCharacter {
    fn default() -> Self {
        let mut t = Timer::new(Duration::from_millis(FIRST_MELEE_TIME), false);
        t.tick(Duration::from_millis(FIRST_MELEE_TIME));
        Self {
            current_lower_animation_state : CharacterAnimationState::Idle,
            character_name: "".to_string(),
            combat_mode : false,
            facing_direction : 0.,
            is_attacking : false,
            next_attack_timer : t,
        }
    }
}
