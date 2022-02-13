
use bevy::prelude::{Commands, Entity, Mut};
use bevy_rapier3d::prelude::{InteractionGroups, RigidBodyActivationComponent, ColliderFlagsComponent, RigidBodyForcesComponent};

use crate::space::core::{rigid_body::components::RigidBodyDisabled, physics::functions::{get_bit_masks, ColliderGroup}};

pub fn disable_rigidbody(
    rigidbody_activation : &mut Mut<RigidBodyActivationComponent>,
    collider_flags : &mut Mut<ColliderFlagsComponent>,
    rigidbody_forces : &mut Mut<RigidBodyForcesComponent>,
    commands : &mut Commands,
    rigidbody_entity : Entity,
) {

    let masks = get_bit_masks(ColliderGroup::NoCollision);

    collider_flags.collision_groups  = InteractionGroups::new(masks.0,masks.1);

    rigidbody_forces.gravity_scale = 0.;

    rigidbody_activation.sleeping = true;

    commands.entity(rigidbody_entity).insert(RigidBodyDisabled);

}

pub fn enable_rigidbody(
    rigidbody_activation : &mut Mut<RigidBodyActivationComponent>,
    collider_flags : &mut Mut<ColliderFlagsComponent>,
    rigidbody_forces : &mut Mut<RigidBodyForcesComponent>,
    commands : &mut Commands,
    rigidbody_entity : Entity,
) {

    let masks = get_bit_masks(ColliderGroup::Standard);

    collider_flags.collision_groups  = InteractionGroups::new(masks.0,masks.1);

    rigidbody_forces.gravity_scale = 1.;

    rigidbody_activation.sleeping = false;

    commands.entity(rigidbody_entity).remove_bundle::<(RigidBodyDisabled,)>();

}
