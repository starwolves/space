use bevy::prelude::Entity;
use resources::math::Vec3Int;

/// Get a desired bit mask as a function.

pub fn get_bit_masks(group: ColliderGroup) -> (u32, u32) {
    match group {
        ColliderGroup::Standard => (
            //membership
            0b00000000000000000000000000000001,
            //filter
            0b00000000000000000000000000000001,
        ),
        ColliderGroup::NoCollision => (
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
        ),
        ColliderGroup::GridmapSelection => (
            0b00000000000000000000000000000010,
            0b00000000000000000000000000000010,
        ),
    }
}

/// Collider groups.

pub enum ColliderGroup {
    NoCollision,
    Standard,
    GridmapSelection,
}

/// Character floor physics friction.

pub const CHARACTER_FLOOR_FRICTION: f32 = 7.2;

/// Reach result.

pub struct ReachResult {
    pub distance: f32,
    pub hit_entity: Option<(Entity, bool)>,
    pub hit_cell: Option<Vec3Int>,
}
