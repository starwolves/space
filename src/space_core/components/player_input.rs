use bevy::math::Vec2;

pub struct PlayerInput {
    pub movement_vector : Vec2,
    pub sprinting : bool,
    pub is_mouse_action_pressed : bool,
}


impl Default for PlayerInput {
    fn default() -> Self {
        Self {
            movement_vector : Vec2::ZERO,
            sprinting : false,
            is_mouse_action_pressed : false,
        }
    }
}
