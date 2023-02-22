use bevy::prelude::{Added, Changed, Commands, Component, Entity, Query};
use bevy_rapier3d::prelude::RigidBodyDisabled;
use math::grid::Vec3Int;

use crate::rigid_body::RigidBodyStatus;

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

/// Disable a rigidbody as a function.

pub(crate) fn disable_rigidbodies(
    mut query: Query<(Entity, &RigidBodyStatus), Changed<RigidBodyStatus>>,
    mut commands: Commands,
    query_added: Query<Entity, Added<RigidBodyStatus>>,
) {
    for (entity, status) in query.iter_mut() {
        if !status.enabled {
            commands.entity(entity).insert(RigidBodyDisabled);
        } else {
            match query_added.get(entity) {
                Ok(_) => {}
                Err(_) => {
                    commands.entity(entity).remove::<RigidBodyDisabled>();
                }
            }
        }
    }
}

/// Reach result.

pub struct ReachResult {
    pub distance: f32,
    pub hit_entity: Option<(Entity, bool)>,
    pub hit_cell: Option<Vec3Int>,
}
/// The component that links and stores rigid body transform.
#[derive(Component)]

pub struct RigidBodyLinkTransform {
    pub follow_entity: Entity,
    pub active: bool,
}

impl Default for RigidBodyLinkTransform {
    fn default() -> Self {
        Self {
            follow_entity: Entity::from_raw(0),
            active: true,
        }
    }
}
