pub struct StandardCharacter {
    pub current_lower_animation_state : CharacterAnimationState,
    pub character_name : String,
    pub combat_mode : bool,
    pub facing_direction : f32,
    pub is_attacking : bool,
}

pub enum CharacterAnimationState {
    Idle,
    Jogging,
    Sprinting,
}

impl Default for StandardCharacter {
    fn default() -> Self {
        Self {
            current_lower_animation_state : CharacterAnimationState::Idle,
            character_name: "".to_string(),
            combat_mode : false,
            facing_direction : 0.,
            is_attacking : false,
        }
    }
}
