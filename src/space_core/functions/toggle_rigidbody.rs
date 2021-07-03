use bevy::prelude::Mut;
use bevy_rapier3d::prelude::{ColliderBundle, ColliderFlags, InteractionGroups, RigidBodyActivation, RigidBodyForces};

use super::collider_interaction_groups::{ColliderGroup, get_bit_masks};

pub fn turn_off_rigidbody(
    rigidbody_activation : &mut Mut<RigidBodyActivation>,
    collider_bundle : &mut Mut<ColliderBundle>,
    rigidbody_forces : &mut Mut<RigidBodyForces>,
) {

    let masks = get_bit_masks(ColliderGroup::NoCollision);

    collider_bundle.flags = ColliderFlags {
        collision_groups: InteractionGroups::new(masks.0,masks.1),
        ..Default::default()
    };

    rigidbody_forces.gravity_scale = 0.;

    rigidbody_activation.sleeping = true;


}
