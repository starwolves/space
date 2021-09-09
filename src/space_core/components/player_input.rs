use bevy::math::Vec2;

pub struct PlayerInput {
    pub movement_vector : Vec2,
    pub sprinting : bool,
    pub is_mouse_action_pressed : bool,
    pub targetted_limb : String,
    pub auto_move_enabled : bool,
    pub auto_move_direction : Vec2,
}


impl Default for PlayerInput {
    fn default() -> Self {
        Self {
            movement_vector : Vec2::ZERO,
            sprinting : false,
            is_mouse_action_pressed : false,
            targetted_limb : "torso".to_string(),
            auto_move_enabled : false,
            auto_move_direction : Vec2::ZERO,
        }
    }
}
