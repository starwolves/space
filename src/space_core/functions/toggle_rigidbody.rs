use bevy::prelude::{Commands, Entity, Mut};
use bevy_rapier3d::prelude::{ColliderFlags, InteractionGroups, RigidBodyActivation, RigidBodyForces};

use crate::space_core::components::rigidbody_disabled::RigidBodyDisabled;

use super::collider_interaction_groups::{ColliderGroup, get_bit_masks};

pub fn disable_rigidbody(
    rigidbody_activation : &mut Mut<RigidBodyActivation>,
    collider_flags : &mut Mut<ColliderFlags>,
    rigidbody_forces : &mut Mut<RigidBodyForces>,
    commands : &mut Commands,
    rigidbody_entity : Entity,
) {

    let masks = get_bit_masks(ColliderGroup::NoCollision);

    collider_flags.collision_groups  = InteractionGroups::new(masks.0,masks.1);

    rigidbody_forces.gravity_scale = 0.;

    rigidbody_activation.sleeping = true;

    commands.entity(rigidbody_entity).insert(RigidBodyDisabled);

}
