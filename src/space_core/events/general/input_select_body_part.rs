use bevy::prelude::Entity;

pub struct InputSelectBodyPart {
    pub handle : u32,
    pub entity : Entity,
    pub body_part : String,
}
