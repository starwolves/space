use crate::{
    physics::{get_bit_masks, ColliderGroup},
    rigid_body::RigidBodyData,
};
use bevy::{
    hierarchy::BuildChildren,
    prelude::{Commands, Entity, EventReader, GlobalTransform, Transform},
};
use bevy_rapier3d::prelude::{
    ActiveEvents, Collider, CollisionGroups, Damping, ExternalForce, ExternalImpulse, Friction,
    GravityScale, Group, RigidBody, Sleeping, Velocity,
};
use entity::spawn::EntityBuildData;

#[cfg(any(feature = "server", feature = "client"))]
pub struct RigidBodyBundle {
    pub collider: Collider,
    pub collider_transform: Transform,
    pub collider_friction: Friction,
    pub rigidbody_dynamic: bool,
    pub collision_events: bool,
}

#[cfg(any(feature = "server", feature = "client"))]
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

#[cfg(any(feature = "server", feature = "client"))]
pub struct RigidBodyBuildData {
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

#[cfg(any(feature = "server", feature = "client"))]
impl Default for RigidBodyBuildData {
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
            collider_collision_groups: CollisionGroups::new(
                Group::from_bits(masks.0).unwrap(),
                Group::from_bits(masks.1).unwrap(),
            ),
            collision_events: false,
        }
    }
}
use entity::entity_data::{WorldMode, WorldModes};

use crate::physics::RigidBodyDisabled;
#[cfg(any(feature = "server", feature = "client"))]
pub fn rigidbody_builder(
    commands: &mut Commands,
    rigidbody_spawn_data: RigidBodyBuildData,
    entity: Entity,
    is_showcase: bool,
) {
    let rigidbody;
    let masks;

    if is_showcase {
        rigidbody = RigidBody::Fixed;
        masks = get_bit_masks(ColliderGroup::NoCollision);
    } else if rigidbody_spawn_data.rigidbody_dynamic {
        rigidbody = RigidBody::Dynamic;
        match rigidbody_spawn_data.entity_is_stored_item {
            true => {
                masks = get_bit_masks(ColliderGroup::NoCollision);
            }
            false => {
                masks = (
                    rigidbody_spawn_data
                        .collider_collision_groups
                        .memberships
                        .bits(),
                    rigidbody_spawn_data
                        .collider_collision_groups
                        .filters
                        .bits(),
                );
            }
        }
    } else {
        rigidbody = RigidBody::Fixed;
        masks = (
            rigidbody_spawn_data
                .collider_collision_groups
                .memberships
                .bits(),
            rigidbody_spawn_data
                .collider_collision_groups
                .filters
                .bits(),
        );
    }

    let mut builder = commands.entity(entity);

    builder
        .insert(GlobalTransform::default())
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
        true => builder.insert((
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
        let mut child_builder = children.spawn(());
        child_builder
            .insert(rigidbody_spawn_data.collider)
            .insert(rigidbody_spawn_data.collider_transform)
            .insert(rigidbody_spawn_data.collider_friction)
            .insert(CollisionGroups::new(
                Group::from_bits(masks.0).unwrap(),
                Group::from_bits(masks.1).unwrap(),
            ));

        if rigidbody_spawn_data.collision_events {
            child_builder.insert(ActiveEvents::COLLISION_EVENTS);
        }
    });

    match rigidbody_spawn_data.entity_is_stored_item {
        true => {
            builder.insert((
                RigidBodyDisabled,
                WorldMode {
                    mode: WorldModes::Worn,
                },
            ));
        }
        false => match rigidbody_spawn_data.rigidbody_dynamic {
            true => {
                builder.insert(WorldMode {
                    mode: WorldModes::Physics,
                });
            }
            false => {}
        },
    }
}

#[cfg(any(feature = "server", feature = "client"))]
pub trait RigidBodyBuilder<Y> {
    fn get_bundle(&self, spawn_data: &EntityBuildData, entity_data_option: Y) -> RigidBodyBundle;
}
use entity::spawn::{NoData, SpawnEntity};

/// Rigid body spawning.
#[cfg(any(feature = "server", feature = "client"))]
pub fn build_rigid_boies<T: RigidBodyBuilder<NoData> + Send + Sync + 'static>(
    mut spawn_events: EventReader<SpawnEntity<T>>,
    mut commands: Commands,
) {
    for spawn_event in spawn_events.iter() {
        let rigidbody_bundle = spawn_event
            .builder
            .get_bundle(&spawn_event.spawn_data, NoData);

        rigidbody_builder(
            &mut commands,
            RigidBodyBuildData {
                rigidbody_dynamic: rigidbody_bundle.rigidbody_dynamic,
                rigid_transform: spawn_event.spawn_data.entity_transform,
                entity_is_stored_item: spawn_event.spawn_data.holder_entity_option.is_some(),
                collider: rigidbody_bundle.collider,
                collider_transform: rigidbody_bundle.collider_transform,
                collider_friction: rigidbody_bundle.collider_friction,
                collision_events: rigidbody_bundle.collision_events,
                ..Default::default()
            },
            spawn_event.spawn_data.entity,
            spawn_event.spawn_data.showcase_data_option.is_some(),
        );
    }
}
