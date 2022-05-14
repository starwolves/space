use std::collections::HashMap;

use bevy_ecs::{
    entity::Entity,
    prelude::Without,
    system::{ParamSet, Query},
};
use bevy_log::warn;
use bevy_transform::components::Transform;

use crate::core::{entity::components::Showcase, rigid_body::components::RigidBodyLinkTransform};

pub fn rigidbody_link_transform(
    mut rigidbodies_set: ParamSet<(
        Query<(Entity, &RigidBodyLinkTransform, &mut Transform), Without<Showcase>>,
        Query<&Transform, Without<Showcase>>,
    )>,
) {
    let mut linked_with_following = HashMap::new();

    for (entity, rigid_body_link_transform_component, _rigidbody_position_component) in
        rigidbodies_set.p0().iter()
    {
        if rigid_body_link_transform_component.active {
            linked_with_following.insert(entity, rigid_body_link_transform_component.follow_entity);
        }
    }

    let mut linked_with_new_positions = HashMap::new();

    let p1 = rigidbodies_set.p1();

    for (linked_entity, following_entity) in linked_with_following {
        let owner_entity_option = p1.get(following_entity);
        match owner_entity_option {
            Ok(owner_entity) => {
                linked_with_new_positions.insert(linked_entity, owner_entity.clone());
            }
            Err(_rr) => {
                warn!("Cannot find follow_entity in the right query.");
            }
        }
    }

    for (entity, rigid_body_link_transform_component, mut rigidbody_position_component) in
        rigidbodies_set.p0().iter_mut()
    {
        if rigid_body_link_transform_component.active {
            let t = *linked_with_new_positions.get_mut(&entity).expect(
                "Couldn't find linked entity that we were about to set following transform of.",
            );

            rigidbody_position_component.translation = t.translation;
            rigidbody_position_component.scale = t.scale;
            rigidbody_position_component.rotation = t.rotation;
        }
    }
}
