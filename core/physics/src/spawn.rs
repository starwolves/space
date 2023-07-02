use crate::{
    physics::{get_bit_masks, ColliderGroup},
    rigid_body::{RigidBodyData, RigidBodyStatus},
};
use bevy::prelude::{Commands, Entity, EventReader, GlobalTransform, Transform};
use bevy_xpbd_3d::prelude::{
    Collider, CollisionLayers, ExternalForce, Friction, LinearVelocity, RigidBody, Sleeping,
};
use entity::spawn::EntityBuildData;

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

pub struct RigidBodyBuildData {
    pub rigidbody_dynamic: bool,
    pub rigid_transform: Transform,
    pub external_force: ExternalForce,
    pub velocity: LinearVelocity,
    pub sleeping: Sleeping,
    pub entity_is_stored_item: bool,
    pub collider: Collider,
    pub collider_transform: Transform,
    pub collider_friction: Friction,
    pub collider_collision_groups: CollisionLayers,
    pub collision_events: bool,
}

impl Default for RigidBodyBuildData {
    fn default() -> Self {
        let masks = get_bit_masks(ColliderGroup::Standard);
        Self {
            rigidbody_dynamic: false,
            rigid_transform: Transform::default(),
            external_force: ExternalForce::default(),
            velocity: LinearVelocity::default(),
            sleeping: Sleeping::default(),
            entity_is_stored_item: false,
            collider: Collider::cuboid(0.2, 0.2, 0.2),
            collider_transform: Transform::default(),
            collider_friction: Friction::default(),
            collider_collision_groups: CollisionLayers::from_bits(masks.0, masks.1),
            collision_events: false,
        }
    }
}
use entity::entity_data::{WorldMode, WorldModes};

pub fn rigidbody_builder(
    commands: &mut Commands,
    rigidbody_spawn_data: RigidBodyBuildData,
    entity: Entity,
    is_showcase: bool,
) {
    let rigidbody;
    let masks;

    if is_showcase {
        rigidbody = RigidBody::Static;
        let m = get_bit_masks(ColliderGroup::NoCollision);
        masks = CollisionLayers::from_bits(m.0, m.1);
    } else if rigidbody_spawn_data.rigidbody_dynamic {
        rigidbody = RigidBody::Dynamic;
        match rigidbody_spawn_data.entity_is_stored_item {
            true => {
                let m = get_bit_masks(ColliderGroup::NoCollision);
                masks = CollisionLayers::from_bits(m.0, m.1);
            }
            false => {
                masks = rigidbody_spawn_data.collider_collision_groups;
            }
        }
    } else {
        rigidbody = RigidBody::Static;
        masks = rigidbody_spawn_data.collider_collision_groups;
    }

    let mut builder = commands.entity(entity);

    builder
        .insert(GlobalTransform::default())
        .insert(rigidbody)
        .insert(rigidbody_spawn_data.rigid_transform)
        .insert(rigidbody_spawn_data.external_force)
        .insert(rigidbody_spawn_data.velocity)
        .insert(RigidBodyData {
            dynamic_friction: rigidbody_spawn_data.collider_friction.dynamic_coefficient,
            static_friction: rigidbody_spawn_data.collider_friction.static_coefficient,
            friction_combine_rule: rigidbody_spawn_data.collider_friction.combine_rule,
        });

    let mut rigidbody_enabled = true;

    if rigidbody_spawn_data.entity_is_stored_item {
        builder.insert(Sleeping);
        rigidbody_enabled = false;
    } else {
        builder
            .insert(Sleeping::default())
            .insert(RigidBodyStatus { enabled: true });
    }
    builder
        .insert(rigidbody_spawn_data.collider)
        .insert(rigidbody_spawn_data.collider_transform)
        .insert(rigidbody_spawn_data.collider_friction)
        .insert(masks);

    match rigidbody_spawn_data.entity_is_stored_item {
        true => {
            builder.insert((WorldMode {
                mode: WorldModes::Worn,
            },));
            rigidbody_enabled = false;
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
    builder.insert(RigidBodyStatus {
        enabled: rigidbody_enabled,
    });
}

pub trait RigidBodyBuilder<Y>: Send + Sync {
    fn get_bundle(&self, spawn_data: &EntityBuildData, entity_data_option: Y) -> RigidBodyBundle;
}
use entity::spawn::{NoData, SpawnEntity};

/// Rigid body spawning.

pub fn build_rigid_bodies<T: RigidBodyBuilder<NoData> + 'static>(
    mut spawn_events: EventReader<SpawnEntity<T>>,
    mut commands: Commands,
) {
    for spawn_event in spawn_events.iter() {
        let rigidbody_bundle = spawn_event
            .entity_type
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
