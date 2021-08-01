use std::collections::HashMap;

use bevy::prelude::{Entity, Query, QuerySet, warn};
use bevy_rapier3d::prelude::RigidBodyPosition;

use crate::space_core::components::rigidbody_link_transform::RigidBodyLinkTransform;

pub fn rigidbody_link_transform(
    mut rigidbodies_set: QuerySet<(
        Query<(Entity, &RigidBodyLinkTransform, &mut RigidBodyPosition)>,
        Query<&RigidBodyPosition>,
    )>,
) {

    let mut linked_with_following = HashMap::new();

    for (entity, rigid_body_link_transform_component, _rigidbody_position_component) in rigidbodies_set.q0_mut().iter_mut() {
        if rigid_body_link_transform_component.active {
            linked_with_following.insert(entity, rigid_body_link_transform_component.follow_entity );
        }
    }

    let mut linked_with_new_positions = HashMap::new();

    for (linked_entity, following_entity) in linked_with_following {
        
        let owner_entity_option = rigidbodies_set.q1().get(following_entity);
        match owner_entity_option {
            Ok(owner_entity) => {
                linked_with_new_positions.insert(linked_entity, owner_entity.position);
            },
            Err(_rr) => {
                warn!("Cannot find follow_entity in the right query.");
            },
        }

    }

    for (entity, rigid_body_link_transform_component, mut rigidbody_position_component) in rigidbodies_set.q0_mut().iter_mut() {
        if rigid_body_link_transform_component.active {
            rigidbody_position_component.position = *linked_with_new_positions.get(&entity).expect("Couldn't find linked entity that we were about to set following transform of.");
        }
    }

}
