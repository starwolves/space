use bevy::prelude::{warn, Entity, Query, Without};
use bevy_rapier3d::prelude::RigidBodyPositionComponent;

use crate::space::{
    core::{
        entity::components::EntityData, gridmap::resources::FOV_MAP_WIDTH,
        rigid_body::components::RigidBodyDisabled,
    },
    entities::{
        air_lock_security::components::AirLock, counter_window_security::components::CounterWindow,
    },
};

pub fn out_of_bounds_check(
    mut rigid_bodies: Query<
        (Entity, &EntityData, &mut RigidBodyPositionComponent),
        (
            Without<AirLock>,
            Without<CounterWindow>,
            Without<RigidBodyDisabled>,
        ),
    >,
) {
    let max = FOV_MAP_WIDTH as f32 * 0.5 * 2.;

    for (rigid_body_entity, entity_data_component, mut rigid_body_position_component) in
        rigid_bodies.iter_mut()
    {
        if rigid_body_position_component.position.translation.y > 5.
            || rigid_body_position_component.position.translation.y < -5.
        {
            warn!(
                "Entity {:?} {} is out of y-axis range at position {}.",
                rigid_body_entity,
                entity_data_component.entity_name,
                rigid_body_position_component.position.translation
            );
            rigid_body_position_component.position.translation.y = 0.5;
        }

        if rigid_body_position_component.position.translation.x > max {
            let overshot = rigid_body_position_component.position.translation.x - max;
            rigid_body_position_component.position.translation.x = -max + overshot;
        }
        if rigid_body_position_component.position.translation.x < -max {
            let overshot = rigid_body_position_component.position.translation.x.abs() - max.abs();
            rigid_body_position_component.position.translation.x = max - overshot;
        }
        if rigid_body_position_component.position.translation.z > max {
            let overshot = rigid_body_position_component.position.translation.z - max;
            rigid_body_position_component.position.translation.z = -max + overshot;
        }
        if rigid_body_position_component.position.translation.z < -max {
            let overshot = rigid_body_position_component.position.translation.z.abs() - max.abs();
            rigid_body_position_component.position.translation.z = max - overshot;
        }
    }
}
