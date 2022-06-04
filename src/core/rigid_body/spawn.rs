use bevy_ecs::{entity::Entity, event::EventReader, system::Commands};
use bevy_hierarchy::BuildChildren;
use bevy_rapier3d::prelude::{
    ActiveEvents, Collider, CollisionGroups, Damping, ExternalForce, ExternalImpulse, Friction,
    GravityScale, RigidBody, Sleeping, Velocity,
};
use bevy_transform::prelude::Transform;

use crate::core::{
    entity::{resources::SpawnData, spawn::SpawnEvent},
    physics::functions::{get_bit_masks, ColliderGroup},
};

use super::components::{RigidBodyData, RigidBodyDisabled};

pub struct RigidBodyBundle {
    pub collider: Collider,
    pub collider_transform: Transform,
    pub collider_friction: Friction,
    pub rigidbody_dynamic: bool,
    pub collision_events: bool,
}

impl Default for RigidBodyBundle {
    fn default() -> Self {
        Self {
            collider: Collider::cuboid(0.2, 0.2, 0.2),
            collider_transform: Transform::default(),
            collider_friction: Friction::default(),
            rigidbody_dynamic: true,
            collision_events: false,
        }
    }
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
    pub collision_events: bool,
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
            collision_events: false,
        }
    }
}

pub fn rigidbody_builder(
    commands: &mut Commands,
    rigidbody_spawn_data: RigidBodySpawnData,
    entity: Entity,
) {
    let rigidbody;
    let masks;

    if rigidbody_spawn_data.rigidbody_dynamic {
        rigidbody = RigidBody::Dynamic;
        match rigidbody_spawn_data.entity_is_stored_item {
            true => {
                masks = get_bit_masks(ColliderGroup::NoCollision);
            }
            false => {
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
        true => builder.insert_bundle((
            GravityScale(0.),
            Sleeping {
                sleeping: true,
                ..Default::default()
            },
            RigidBodyDisabled,
            Damping {
                linear_damping: 10000.,
                angular_damping: 10000.,
            },
        )),
        false => builder
            .insert(Sleeping::default())
            .insert(rigidbody_spawn_data.gravity_scale)
            .insert(Damping::default()),
    }
    .with_children(|children| {
        let mut child_builder = children.spawn();
        child_builder
            .insert(rigidbody_spawn_data.collider)
            .insert(rigidbody_spawn_data.collider_transform)
            .insert(rigidbody_spawn_data.collider_friction)
            .insert(CollisionGroups::new(masks.0, masks.1));

        if rigidbody_spawn_data.collision_events {
            child_builder.insert(ActiveEvents::COLLISION_EVENTS);
        }
    });
}

pub trait RigidBodySummonable {
    fn get_bundle(&self, spawn_data: &SpawnData) -> RigidBodyBundle;
}

pub fn summon_rigid_body<T: RigidBodySummonable + Send + Sync + 'static>(
    mut spawn_events: EventReader<SpawnEvent<T>>,
    mut commands: Commands,
) {
    for spawn_event in spawn_events.iter() {
        let rigidbody_bundle = spawn_event.summoner.get_bundle(&spawn_event.spawn_data);

        rigidbody_builder(
            &mut commands,
            RigidBodySpawnData {
                rigidbody_dynamic: rigidbody_bundle.rigidbody_dynamic,
                rigid_transform: spawn_event.spawn_data.entity_transform,
                entity_is_stored_item: spawn_event.spawn_data.held_data_option.is_some(),
                collider: rigidbody_bundle.collider,
                collider_transform: rigidbody_bundle.collider_transform,
                collider_friction: rigidbody_bundle.collider_friction,
                collision_events: rigidbody_bundle.collision_events,
                ..Default::default()
            },
            spawn_event.spawn_data.entity,
        );
    }
}
