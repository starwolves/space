use bevy::prelude::{Query, warn, Entity, Without};
use bevy_rapier3d::prelude::RigidBodyPositionComponent;

use crate::space::{core::{gridmap::resources::FOV_MAP_WIDTH, entity::components::EntityData}};

pub fn out_of_bounds_check(

    rigid_bodies : Query<(Entity, &EntityData, &RigidBodyPositionComponent), Without<RigidBodyPositionComponent>>,

) {

    let max = FOV_MAP_WIDTH as f32 * 0.5 * 2.;

    for (rigid_body_entity, entity_data_component, rigid_body_position_component) in rigid_bodies.iter() {

        if rigid_body_position_component.position.translation.y > 5.
        || rigid_body_position_component.position.translation.y < -5.
        || rigid_body_position_component.position.translation.x > max 
        || rigid_body_position_component.position.translation.x < -max
        || rigid_body_position_component.position.translation.z > max
        || rigid_body_position_component.position.translation.z < -max {

            warn!("Entity {:?} {} is out of range at position {}.", rigid_body_entity, entity_data_component.entity_type, rigid_body_position_component.position.translation);

        }

    }

}
