use bevy::{math::Vec3, prelude::Commands};
use bevy_rapier3d::prelude::{CoefficientCombineRule, ColliderBundle, ColliderFlags, ColliderMaterial, ColliderShape, ColliderType, InteractionGroups, RigidBodyBundle, RigidBodyCcd, RigidBodyType};

use crate::space_core::functions::entity::collider_interaction_groups::{ColliderGroup, get_bit_masks};

pub fn build_gridmap_floor(
    commands : &mut Commands,
) {

    let masks = get_bit_masks(ColliderGroup::Standard);

    commands.spawn_bundle(RigidBodyBundle {
        body_type: RigidBodyType::Static,
        position: Vec3::new(0.,-1.,0.).into(),
        ccd: RigidBodyCcd {
            ccd_enabled: false,
            ..Default::default()
        },
        ..Default::default()
    }).insert_bundle(
        ColliderBundle {
            shape: ColliderShape::cuboid(500., 1., 500.),
            collider_type: ColliderType::Solid,
            material: ColliderMaterial {
                friction_combine_rule:  CoefficientCombineRule::Average,
                friction: 0.5,
                ..Default::default()
            },
            flags: ColliderFlags {
                collision_groups: InteractionGroups::new(masks.0,masks.1),
                ..Default::default()
            },
            ..Default::default()
        }
    );

}
