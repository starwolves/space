use bevy_ecs::{entity::Entity, system::Commands};
use bevy_hierarchy::BuildChildren;
use bevy_rapier3d::prelude::{
    Collider, CollisionGroups, ExternalForce, ExternalImpulse, Friction, GravityScale, RigidBody,
    Sleeping, Velocity,
};
use bevy_transform::prelude::Transform;

use crate::core::physics::functions::{get_bit_masks, ColliderGroup};

use super::components::RigidBodyData;

pub struct RigidbodyBundle {
    pub collider: Collider,
    pub collider_transform: Transform,
    pub collider_friction: Friction,
}

pub struct RigidBodySpawnData {
    pub rigidbody_dynamic: bool,
    pub rigid_transform: Transform,
    pub external_impulse: ExternalImpulse,
    pub external_force: ExternalForce,
    pub velocity: Velocity,
    pub gravity_scale: GravityScale,
    pub sleeping: Sleeping,
    pub entity_is_stored_item: bool,
    pub collider: Collider,
    pub collider_transform: Transform,
    pub collider_friction: Friction,
    pub collider_collision_groups: CollisionGroups,
}

impl Default for RigidBodySpawnData {
    fn default() -> Self {
        let masks = get_bit_masks(ColliderGroup::Standard);
        Self {
            rigidbody_dynamic: false,
            rigid_transform: Transform::default(),
            external_impulse: ExternalImpulse::default(),
            external_force: ExternalForce::default(),
            velocity: Velocity::default(),
            gravity_scale: GravityScale::default(),
            sleeping: Sleeping::default(),
            entity_is_stored_item: false,
            collider: Collider::cuboid(0.2, 0.2, 0.2),
            collider_transform: Transform::default(),
            collider_friction: Friction::default(),
            collider_collision_groups: CollisionGroups::new(masks.0, masks.1),
        }
    }
}

pub fn rigidbody_builder(
    commands: &mut Commands,
    entity: Entity,
    rigidbody_spawn_data: RigidBodySpawnData,
) {
    let rigidbody;
    let masks;

    if rigidbody_spawn_data.rigidbody_dynamic {
        match rigidbody_spawn_data.entity_is_stored_item {
            true => {
                rigidbody = RigidBody::Fixed;
                masks = get_bit_masks(ColliderGroup::NoCollision);
            }
            false => {
                rigidbody = RigidBody::Dynamic;
                masks = (
                    rigidbody_spawn_data.collider_collision_groups.memberships,
                    rigidbody_spawn_data.collider_collision_groups.filters,
                );
            }
        }
    } else {
        rigidbody = RigidBody::Fixed;
        masks = (
            rigidbody_spawn_data.collider_collision_groups.memberships,
            rigidbody_spawn_data.collider_collision_groups.filters,
        );
    }

    let mut builder = commands.entity(entity);

    builder
        .insert(rigidbody)
        .insert(rigidbody_spawn_data.rigid_transform)
        .insert(rigidbody_spawn_data.external_impulse)
        .insert(rigidbody_spawn_data.external_force)
        .insert(rigidbody_spawn_data.velocity)
        .insert(RigidBodyData {
            friction: rigidbody_spawn_data.collider_friction.coefficient,
            friction_combine_rule: rigidbody_spawn_data.collider_friction.combine_rule,
        });

    match rigidbody_spawn_data.entity_is_stored_item {
        true => builder.insert(GravityScale(0.)).insert(Sleeping {
            sleeping: true,
            ..Default::default()
        }),
        false => builder
            .insert(Sleeping::default())
            .insert(rigidbody_spawn_data.gravity_scale),
    }
    .with_children(|children| {
        children
            .spawn()
            .insert(rigidbody_spawn_data.collider)
            .insert(rigidbody_spawn_data.collider_transform)
            .insert(rigidbody_spawn_data.collider_friction)
            .insert(CollisionGroups::new(masks.0, masks.1));
    });
}
