pub struct StandardCharacter {
    pub current_animation_state : State,
    pub character_name : String,
}

pub enum State {
    Idle,
    Walking,
    Sprinting,
}
