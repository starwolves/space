use bevy_internal::{math::Vec3, prelude::Commands};
use bevy_rapier3d::prelude::{
    CoefficientCombineRule, ColliderBundle, ColliderFlags, ColliderMaterial, ColliderShape,
    ColliderType, InteractionGroups, RigidBodyBundle, RigidBodyType,
};

use crate::space::{
    core::physics::functions::{get_bit_masks, ColliderGroup},
    entities::human_male_pawn::spawn::CHARACTER_FLOOR_FRICTION,
};

pub fn build_gridmap_floor(commands: &mut Commands) {
    let masks = get_bit_masks(ColliderGroup::Standard);

    //Floor
    commands
        .spawn_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            position: Vec3::new(0., -1., 0.).into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(500., 1., 500.).into(),
            collider_type: ColliderType::Solid.into(),
            material: ColliderMaterial {
                friction_combine_rule: CoefficientCombineRule::Average,
                friction: CHARACTER_FLOOR_FRICTION,
                ..Default::default()
            }
            .into(),
            flags: ColliderFlags {
                collision_groups: InteractionGroups::new(masks.0, masks.1),
                ..Default::default()
            }
            .into(),
            ..Default::default()
        });

    //Roof
    commands
        .spawn_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            position: Vec3::new(0., 3., 0.).into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(500., 1., 500.).into(),
            collider_type: ColliderType::Solid.into(),
            material: ColliderMaterial {
                friction_combine_rule: CoefficientCombineRule::Min,
                friction: CHARACTER_FLOOR_FRICTION,
                ..Default::default()
            }
            .into(),
            flags: ColliderFlags {
                collision_groups: InteractionGroups::new(masks.0, masks.1),
                ..Default::default()
            }
            .into(),
            ..Default::default()
        });
}
