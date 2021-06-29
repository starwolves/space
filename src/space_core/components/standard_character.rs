pub struct StandardCharacter {
    pub current_animation_state : State,
    pub billboard_messages : Vec<String>,
}

pub enum State {
    Idle,
    Walking
}
