use bevy::prelude::Mut;
use bevy_rapier3d::prelude::{ColliderFlags, InteractionGroups, RigidBodyActivation, RigidBodyForces};

use super::collider_interaction_groups::{ColliderGroup, get_bit_masks};

pub fn turn_off_rigidbody(
    rigidbody_activation : &mut Mut<RigidBodyActivation>,
    collider_flags : &mut Mut<ColliderFlags>,
    rigidbody_forces : &mut Mut<RigidBodyForces>,
) {

    let masks = get_bit_masks(ColliderGroup::NoCollision);

    collider_flags.collision_groups  = InteractionGroups::new(masks.0,masks.1);

    rigidbody_forces.gravity_scale = 0.;

    rigidbody_activation.sleeping = true;


}
