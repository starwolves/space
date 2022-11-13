use bevy::prelude::{Commands, Component, Entity};
use bevy_rapier3d::prelude::{CollisionGroups, Damping, GravityScale, Group, Sleeping};
use math::grid::Vec3Int;

/// Get a desired bit mask as a function.
#[cfg(feature = "server")]
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
#[cfg(feature = "server")]
pub enum ColliderGroup {
    NoCollision,
    Standard,
}

/// Character floor physics friction.
#[cfg(feature = "server")]
pub const CHARACTER_FLOOR_FRICTION: f32 = 7.2;

/// Component, an entity has this when its physics is disabled.
#[derive(Component)]
#[cfg(feature = "server")]
pub struct RigidBodyDisabled;
/// Disable a rigidbody as a function.
#[cfg(feature = "server")]
pub fn disable_rigidbody(
    rigidbody_activation: &mut Sleeping,
    collider_flags: &mut CollisionGroups,
    mut gravity: &mut GravityScale,
    commands: &mut Commands,
    rigidbody_entity: Entity,
    damping: &mut Damping,
) {
    let masks = get_bit_masks(ColliderGroup::NoCollision);
    collider_flags.memberships = Group::from_bits(masks.0).unwrap();
    collider_flags.filters = Group::from_bits(masks.1).unwrap();

    gravity.0 = 0.;

    rigidbody_activation.sleeping = true;

    damping.angular_damping = 10000.;
    damping.linear_damping = 10000.;

    commands.entity(rigidbody_entity).insert(RigidBodyDisabled);
}

/// Enable a rigidbody as a function.
#[cfg(feature = "server")]
pub fn enable_rigidbody(
    rigidbody_activation: &mut Sleeping,
    collider_flags: &mut CollisionGroups,
    mut gravity: &mut GravityScale,
    commands: &mut Commands,
    rigidbody_entity: Entity,
    damping: &mut Damping,
) {
    let masks = get_bit_masks(ColliderGroup::Standard);

    collider_flags.memberships = Group::from_bits(masks.0).unwrap();
    collider_flags.filters = Group::from_bits(masks.1).unwrap();

    gravity.0 = 1.;

    rigidbody_activation.sleeping = false;

    damping.angular_damping = 0.;
    damping.linear_damping = 0.;

    commands
        .entity(rigidbody_entity)
        .remove::<RigidBodyDisabled>();
}

/// Reach result.
#[cfg(feature = "server")]
pub struct ReachResult {
    pub distance: f32,
    pub hit_entity: Option<(Entity, bool)>,
    pub hit_cell: Option<Vec3Int>,
}
/// The component that links and stores rigid body transform.
#[derive(Component)]
#[cfg(feature = "server")]
pub struct RigidBodyLinkTransform {
    pub follow_entity: Entity,
    pub active: bool,
}
#[cfg(feature = "server")]
impl Default for RigidBodyLinkTransform {
    fn default() -> Self {
        Self {
            follow_entity: Entity::from_raw(0),
            active: true,
        }
    }
}
