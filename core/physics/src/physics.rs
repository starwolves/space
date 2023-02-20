use bevy::prelude::{Changed, Component, Entity, Query};
use bevy_rapier3d::prelude::{CollisionGroups, Damping, GravityScale, Group, Sleeping};
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
    }
}

/// Collider groups.

pub enum ColliderGroup {
    NoCollision,
    Standard,
}

/// Character floor physics friction.

pub const CHARACTER_FLOOR_FRICTION: f32 = 7.2;

/// Disable a rigidbody as a function.

pub(crate) fn disable_rigidbodies(
    mut query: Query<
        (
            &mut Sleeping,
            &mut CollisionGroups,
            &mut GravityScale,
            &mut Damping,
            &RigidBodyStatus,
        ),
        Changed<RigidBodyStatus>,
    >,
) {
    for (mut rigidbody_activation, mut collider_flags, mut gravity, mut damping, status) in
        query.iter_mut()
    {
        if !status.enabled {
            let masks = get_bit_masks(ColliderGroup::NoCollision);
            collider_flags.memberships = Group::from_bits(masks.0).unwrap();
            collider_flags.filters = Group::from_bits(masks.1).unwrap();

            gravity.0 = 0.;

            rigidbody_activation.sleeping = true;

            damping.angular_damping = 10000.;
            damping.linear_damping = 10000.;
        } else {
            let masks = get_bit_masks(ColliderGroup::Standard);

            collider_flags.memberships = Group::from_bits(masks.0).unwrap();
            collider_flags.filters = Group::from_bits(masks.1).unwrap();

            gravity.0 = 1.;

            rigidbody_activation.sleeping = false;

            damping.angular_damping = 0.;
            damping.linear_damping = 0.;
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
