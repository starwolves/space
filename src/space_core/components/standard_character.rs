pub struct StandardCharacter {
    pub current_animation_state : CharacterAnimationState,
    pub character_name : String,
}

pub enum CharacterAnimationState {
    Idle,
    Walking,
    Sprinting,
}
