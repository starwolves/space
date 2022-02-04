use bevy::prelude::Component;

use super::resources::Vec3Int;

#[derive(Component)]
pub struct Cell {
    pub id : Vec3Int,
}
