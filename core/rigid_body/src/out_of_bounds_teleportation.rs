use api::{entity_updates::EntityData, gridmap::FOV_MAP_WIDTH};
use bevy::prelude::{warn, Entity, Query, Transform, Without};
use physics::physics::RigidBodyDisabled;

/// Check if rigidbody is out of bounds if so teleport on the mirrored side.
pub(crate) fn out_of_bounds_tp(
    mut rigid_bodies: Query<(Entity, &EntityData, &mut Transform), (Without<RigidBodyDisabled>,)>,
) {
    let max = FOV_MAP_WIDTH as f32 * 0.5 * 2.;

    for (rigid_body_entity, entity_data_component, mut rigid_body_position_component) in
        rigid_bodies.iter_mut()
    {
        if rigid_body_position_component.translation.y > 5.
            || rigid_body_position_component.translation.y < -5.
        {
            warn!(
                "Entity {:?} {} is out of y-axis range at position {}.",
                rigid_body_entity,
                entity_data_component.entity_name,
                rigid_body_position_component.translation
            );
            rigid_body_position_component.translation.y = 0.5;
        }

        if rigid_body_position_component.translation.x > max {
            let overshot = rigid_body_position_component.translation.x - max;
            rigid_body_position_component.translation.x = -max + overshot;
        }
        if rigid_body_position_component.translation.x < -max {
            let overshot = rigid_body_position_component.translation.x.abs() - max.abs();
            rigid_body_position_component.translation.x = max - overshot;
        }
        if rigid_body_position_component.translation.z > max {
            let overshot = rigid_body_position_component.translation.z - max;
            rigid_body_position_component.translation.z = -max + overshot;
        }
        if rigid_body_position_component.translation.z < -max {
            let overshot = rigid_body_position_component.translation.z.abs() - max.abs();
            rigid_body_position_component.translation.z = max - overshot;
        }
    }
}
