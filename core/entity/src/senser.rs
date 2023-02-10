use bevy::prelude::{Component, Entity};
use doryen_fov::FovRecursiveShadowCasting;
use math::grid::Vec2Int;

/// Used to check if entities are authorized to examine and obtain certain additional data.
#[derive(PartialEq)]

pub enum SensingAbility {
    AtmosphericsSensor,
    ShipEngineerKnowledge,
}

/// The component of entities that can sense other entities.
#[derive(Component)]

pub struct Senser {
    pub cell_id: Vec2Int,
    pub fov: FovRecursiveShadowCasting,
    pub sensing: Vec<Entity>,
    pub sfx: Vec<Entity>,
    pub sensing_abilities: Vec<SensingAbility>,
}

impl Default for Senser {
    fn default() -> Self {
        Self {
            cell_id: Vec2Int { x: 0, y: 0 },
            fov: FovRecursiveShadowCasting::new(WORLD_WIDTH_CELLS, WORLD_WIDTH_CELLS),
            sensing: vec![],
            sfx: vec![],
            sensing_abilities: vec![],
        }
    }
}

/// Turning up these values drastically increases fov calculation time.
/// Dividible by 2.

pub const WORLD_WIDTH_CELLS: usize = 1024;

/// Use this to use the Doryen FOV algorithm.

pub fn to_doryen_coordinates(x: i16, y: i16) -> (usize, usize) {
    let mut n_x = x + WORLD_WIDTH_CELLS as i16 / 2;
    let mut n_y = y + WORLD_WIDTH_CELLS as i16 / 2;

    if doryen_coordinates_out_of_range(n_x as usize, n_y as usize) {
        n_x = 0;
        n_y = 0;
    }

    (n_x as usize, n_y as usize)
}
/// Check if supplied doryen coordinates are out of range as a function.

pub fn doryen_coordinates_out_of_range(x: usize, y: usize) -> bool {
    x > WORLD_WIDTH_CELLS || y > WORLD_WIDTH_CELLS
}
