use bevy::prelude::Entity;


pub struct RconAuthorization {
    pub handle : u32,
    pub entity : Entity,
    pub input_password : String,
}
