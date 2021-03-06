use api::data::Vec3Int;
use bevy::{math::Vec3, prelude::Entity};

#[derive(Clone)]
pub struct Attack {
    pub attacker: Entity,
    pub weapon_option: Option<Entity>,
    pub targetted_entity: Option<Entity>,
    pub targetted_cell: Option<Vec3Int>,
    pub incremented_id: u64,
    pub angle: f32,
    pub targetted_limb: String,
    pub alt_attack_mode: bool,
}

#[derive(Clone)]
pub struct QueryCombatHitResult {
    pub incremented_id: u64,
    pub entities_hits: Vec<EntityHit>,
    pub cell_hits: Vec<CellHit>,
}

#[derive(Clone)]
pub struct EntityHit {
    pub entity: Entity,
    pub hit_point: Vec3,
}
#[derive(Clone)]
pub struct CellHit {
    pub cell: Vec3Int,
    pub hit_point: Vec3,
}
