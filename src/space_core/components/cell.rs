use bevy::prelude::Component;

use crate::space_core::resources::doryen_fov::Vec3Int;
#[derive(Component)]
pub struct Cell {
    pub id : Vec3Int,
}
