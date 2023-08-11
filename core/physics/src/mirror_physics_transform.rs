use entity::showcase::Showcase;
use std::collections::HashMap;

use bevy::prelude::{warn, Component, Entity, Query, Transform, Without};

/// Manage and link rigid body transform.

pub(crate) fn rigidbody_link_transform(
    mut linked_rigidbodies: Query<(Entity, &MirrorPhysicsTransform), Without<Showcase>>,
    mut transforms: Query<&mut Transform>,
) {
    let mut linked_with_following = HashMap::new();

    for (entity, rigid_body_link_transform_component) in linked_rigidbodies.iter() {
        linked_with_following.insert(
            entity,
            rigid_body_link_transform_component.mirrored_physics_entity,
        );
    }

    let mut linked_with_new_positions = HashMap::new();

    for (linked_entity, following_entity) in linked_with_following {
        let owner_entity_option = transforms.get(following_entity);
        match owner_entity_option {
            Ok(owner_entity) => {
                linked_with_new_positions.insert(linked_entity, owner_entity.clone());
            }
            Err(_rr) => {
                warn!("Cannot find follow_entity in the right query.");
            }
        }
    }

    for (entity, _rigid_body_link_transform_component) in linked_rigidbodies.iter_mut() {
        let t = *linked_with_new_positions.get_mut(&entity).expect(
            "Couldn't find linked entity that we were about to set following transform of.",
        );

        match transforms.get_mut(entity) {
            Ok(mut rigidbody_position_component) => {
                rigidbody_position_component.translation = t.translation;
                rigidbody_position_component.scale = t.scale;
                rigidbody_position_component.rotation = t.rotation;
            }
            Err(_rr) => {
                warn!("Couldnt find transform of linked rigidbody.");
            }
        }
    }
}

/// The component that links and stores rigid body transform.
#[derive(Component)]

pub struct MirrorPhysicsTransform {
    pub mirrored_physics_entity: Entity,
}

impl Default for MirrorPhysicsTransform {
    fn default() -> Self {
        Self {
            mirrored_physics_entity: Entity::from_raw(0),
        }
    }
}
