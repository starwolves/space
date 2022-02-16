use bevy::prelude::Component;

use super::resources::Vec3Int;

#[derive(Component)]
pub struct Cell {
    pub id: Vec3Int,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            id: Vec3Int { x: 0, y: 0, z: 0 },
        }
    }
}
