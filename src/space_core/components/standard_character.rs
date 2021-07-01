pub struct StandardCharacter {
    pub current_animation_state : State,
}

pub enum State {
    Idle,
    Walking,
    Sprinting,
}
