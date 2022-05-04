use bevy_ecs::{entity::Entity, system::Commands};
use bevy_rapier3d::prelude::{CollisionGroups, GravityScale, Sleeping};

use crate::core::{
    physics::functions::{get_bit_masks, ColliderGroup},
    rigid_body::components::RigidBodyDisabled,
};

pub fn disable_rigidbody(
    rigidbody_activation: &mut Sleeping,
    collider_flags: &mut CollisionGroups,
    mut gravity: &mut GravityScale,
    commands: &mut Commands,
    rigidbody_entity: Entity,
) {
    let masks = get_bit_masks(ColliderGroup::NoCollision);

    collider_flags.memberships = masks.0;
    collider_flags.filters = masks.1;

    gravity.0 = 0.;

    rigidbody_activation.sleeping = true;

    commands.entity(rigidbody_entity).insert(RigidBodyDisabled);
}

pub fn enable_rigidbody(
    rigidbody_activation: &mut Sleeping,
    collider_flags: &mut CollisionGroups,
    mut gravity: &mut GravityScale,
    commands: &mut Commands,
    rigidbody_entity: Entity,
) {
    let masks = get_bit_masks(ColliderGroup::Standard);

    collider_flags.memberships = masks.0;
    collider_flags.filters = masks.1;

    gravity.0 = 1.;

    rigidbody_activation.sleeping = false;

    commands
        .entity(rigidbody_entity)
        .remove_bundle::<(RigidBodyDisabled,)>();
}
