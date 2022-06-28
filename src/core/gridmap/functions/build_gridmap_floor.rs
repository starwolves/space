use bevy_ecs::system::Commands;
use bevy_math::Vec3;
use bevy_rapier3d::prelude::{
    CoefficientCombineRule, Collider, CollisionGroups, Friction, RigidBody,
};
use bevy_transform::prelude::{GlobalTransform, Transform};

use crate::{
    core::physics::functions::{get_bit_masks, ColliderGroup},
    entities::human_male::spawn::CHARACTER_FLOOR_FRICTION,
};

pub fn build_gridmap_floor(commands: &mut Commands) {
    let masks = get_bit_masks(ColliderGroup::Standard);

    //Floor

    let mut friction_component = Friction::coefficient(CHARACTER_FLOOR_FRICTION);
    friction_component.combine_rule = CoefficientCombineRule::Average;

    commands
        .spawn()
        .insert(RigidBody::Fixed)
        .insert(Transform::from_translation(Vec3::new(0., -1., 0.)))
        .insert(GlobalTransform::default())
        .insert(Collider::cuboid(500., 1., 500.))
        .insert(friction_component)
        .insert(CollisionGroups::new(masks.0, masks.1));

    //Roof

    let mut friction_component = Friction::coefficient(CHARACTER_FLOOR_FRICTION);
    friction_component.combine_rule = CoefficientCombineRule::Min;

    commands
        .spawn()
        .insert(RigidBody::Fixed)
        .insert(Transform::from_translation(Vec3::new(0., 3., 0.)))
        .insert(GlobalTransform::default())
        .insert(Collider::cuboid(500., 1., 500.))
        .insert(friction_component)
        .insert(CollisionGroups::new(masks.0, masks.1));
}
