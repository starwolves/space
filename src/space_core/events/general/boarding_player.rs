use bevy::prelude::Entity;


pub struct BoardingPlayer {
    pub player_handle : u32,
    pub player_character_name : String,
    pub entity : Entity,
}
