pub struct StandardCharacter {
    pub current_animation_state : CharacterAnimationState,
    pub character_name : String,
    pub combat_mode : bool,
    pub facing_direction : f32,
}

pub enum CharacterAnimationState {
    Idle,
    Jogging,
    Sprinting,
}

impl Default for StandardCharacter {
    fn default() -> Self {
        Self {
            current_animation_state : CharacterAnimationState::Idle,
            character_name: "".to_string(),
            combat_mode : false,
            facing_direction : 0.,
        }
    }
}
