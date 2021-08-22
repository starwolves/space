pub struct StandardCharacter {
    pub current_animation_state : CharacterAnimationState,
    pub character_name : String,
    pub combat_mode : bool,
}

pub enum CharacterAnimationState {
    Idle,
    Walking,
    Sprinting,
}

impl Default for StandardCharacter {
    fn default() -> Self {
        Self {
            current_animation_state : CharacterAnimationState::Idle,
            character_name: "".to_string(),
            combat_mode : false,
        }
    }
}
