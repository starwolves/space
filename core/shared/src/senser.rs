use bevy::prelude::{Component, Entity};
use doryen_fov::FovRecursiveShadowCasting;

use crate::{data::Vec2Int, gridmap::FOV_MAP_WIDTH};

#[derive(PartialEq)]
pub enum SensingAbility {
    AtmosphericsSensor,
    ShipEngineerKnowledge,
}

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
            fov: FovRecursiveShadowCasting::new(FOV_MAP_WIDTH, FOV_MAP_WIDTH),
            sensing: vec![],
            sfx: vec![],
            sensing_abilities: vec![],
        }
    }
}
