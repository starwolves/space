use bevy::math::Vec2;

#[allow(dead_code)]
pub struct PlayerInput {
    pub movement_vector : Vec2,
    pub sprinting : bool
}


impl Default for PlayerInput {
    fn default() -> Self {
        Self {
            movement_vector : Vec2::ZERO,
            sprinting : false,
        }
    }
}
