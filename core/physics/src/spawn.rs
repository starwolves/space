use std::collections::HashMap;

use crate::{
    entity::{RigidBodies, RigidBodyLink, SFRigidBody},
    physics::{get_bit_masks, ColliderGroup},
    rigid_body::RigidBodyData,
};
use bevy::{
    ecs::system::Resource,
    prelude::{Commands, Entity, EventReader, Res, ResMut, Transform},
    transform::TransformBundle,
};
use bevy_xpbd_3d::{
    components::{
        AngularDamping, AngularVelocity, ExternalAngularImpulse, ExternalImpulse, ExternalTorque,
        LinearDamping,
    },
    prelude::{
        Collider, CollisionLayers, ExternalForce, Friction, LinearVelocity, LockedAxes, RigidBody,
        Sleeping,
    },
};
use entity::spawn::EntityBuildData;

pub struct RigidBodyBundle {
    pub collider: Collider,
    pub collider_transform: Transform,
    pub collider_friction: Friction,
    pub locked_axes: LockedAxes,
    pub rigidbody_dynamic: bool,
    pub collision_events: bool,
    pub external_force: ExternalForce,
    pub mesh_offset: Transform,
}

impl Default for RigidBodyBundle {
    fn default() -> Self {
        Self {
            collider: Collider::cuboid(0.2, 0.2, 0.2),
            collider_transform: Transform::default(),
            collider_friction: Friction::default(),
            rigidbody_dynamic: true,
            collision_events: false,
            locked_axes: LockedAxes::new(),
            external_force: ExternalForce::default(),
            mesh_offset: Transform::default(),
        }
    }
}

pub struct RigidBodyBuildData {
    pub rigidbody_dynamic: bool,
    pub rigid_transform: Transform,
    pub external_force: ExternalForce,
    pub linear_velocity: LinearVelocity,
    pub sleeping: Option<Sleeping>,
    pub entity_is_stored_item: bool,
    pub collider: Collider,
    pub friction: Friction,
    pub collider_collision_layers: CollisionLayers,
    pub collision_events: bool,
    pub mesh_offset: Transform,
    pub locked_axes: LockedAxes,
    pub linear_damping: LinearDamping,
    pub angular_damping: AngularDamping,
    pub angular_velocity: AngularVelocity,
    pub external_torque: ExternalTorque,
    pub external_angular_impulse: ExternalAngularImpulse,
    pub external_impulse: ExternalImpulse,
}

impl Default for RigidBodyBuildData {
    fn default() -> Self {
        let masks = get_bit_masks(ColliderGroup::Standard);
        Self {
            rigidbody_dynamic: false,
            rigid_transform: Transform::default(),
            external_force: ExternalForce::default(),
            linear_velocity: LinearVelocity::default(),
            sleeping: None,
            entity_is_stored_item: false,
            collider: Collider::cuboid(0.2, 0.2, 0.2),
            friction: Friction::default(),
            collider_collision_layers: CollisionLayers::from_bits(masks.0, masks.1),
            collision_events: false,
            locked_axes: LockedAxes::new(),
            mesh_offset: Transform::default(),
            linear_damping: LinearDamping::default(),
            angular_damping: AngularDamping::default(),
            angular_velocity: AngularVelocity::default(),
            external_torque: ExternalTorque::default(),
            external_angular_impulse: ExternalAngularImpulse::default(),
            external_impulse: ExternalImpulse::default(),
        }
    }
}
use entity::entity_data::{WorldMode, WorldModes};

pub fn rigidbody_builder(
    commands: &mut Commands,
    rigidbody_spawn_data: RigidBodyBuildData,
    entity: Entity,
    is_showcase: bool,
    rigidbodies: &mut ResMut<RigidBodies>,
    app_mode: &Res<Mode>,
) {
    let correction_mode = matches!(**app_mode, Mode::Correction);
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
                masks = rigidbody_spawn_data.collider_collision_layers;
            }
        }
    } else {
        rigidbody = RigidBody::Static;
        masks = rigidbody_spawn_data.collider_collision_layers;
    }
    let mut t = TransformBundle::from(rigidbody_spawn_data.rigid_transform);
    let mut builder;
    if correction_mode {
        builder = commands.entity(entity);
    } else {
        builder = commands.spawn(());
    }
    builder.insert((
        t.clone(),
        rigidbody,
        rigidbody_spawn_data.external_force,
        rigidbody_spawn_data.linear_velocity,
        RigidBodyData {
            dynamic_friction: rigidbody_spawn_data.friction.dynamic_coefficient,
            static_friction: rigidbody_spawn_data.friction.static_coefficient,
            friction_combine_rule: rigidbody_spawn_data.friction.combine_rule,
        },
        rigidbody_spawn_data.collider,
        rigidbody_spawn_data.friction,
        masks,
        SFRigidBody,
        rigidbody_spawn_data.locked_axes,
        rigidbody_spawn_data.linear_damping,
        rigidbody_spawn_data.angular_damping,
        rigidbody_spawn_data.angular_velocity,
        rigidbody_spawn_data.external_torque,
        rigidbody_spawn_data.external_angular_impulse,
    ));
    builder.insert((rigidbody_spawn_data.external_impulse,));

    let rigid_entity = builder.id();

    if rigidbody_spawn_data.entity_is_stored_item {
        builder.insert(Sleeping);
    }

    if !(is_server() || correction_mode) {
        t.local.translation += rigidbody_spawn_data.mesh_offset.translation;
        t.local.scale = rigidbody_spawn_data.mesh_offset.scale;
        t.local.rotation *= rigidbody_spawn_data.mesh_offset.rotation;
    }

    if !correction_mode {
        builder = commands.entity(entity);
        builder.insert((
            t.clone(),
            RigidBodyLink {
                offset: rigidbody_spawn_data.mesh_offset,
                target_transform: t.local.clone(),
                origin_transfom: t.local,
                ..Default::default()
            },
        ));
    }

    let mut rigidbody_enabled = true;

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
    if !correction_mode {
        builder = commands.entity(rigid_entity);
    }
    if !rigidbody_enabled {
        builder.insert(Sleeping);
    }

    rigidbodies.link_entity(entity, rigid_entity)
}

pub trait RigidBodyBuilder<Y>: Send + Sync {
    fn get_bundle(&self, spawn_data: &EntityBuildData, entity_data_option: Y) -> RigidBodyBundle;
}
use entity::spawn::{NoData, SpawnEntity};
use networking::stamp::TickRateStamp;
use resources::{
    correction::MAX_CACHE_TICKS_AMNT,
    modes::{is_server, Mode},
    physics::PhysicsSpawn,
};

#[derive(Resource, Default)]
pub struct NewlySpawnedRigidbodies {
    pub cache: HashMap<u64, HashMap<Entity, PhysicsSpawn>>,
}
pub(crate) fn clear_new(mut cache: ResMut<NewlySpawnedRigidbodies>, stamp: Res<TickRateStamp>) {
    let mut to_remove = vec![];
    for recorded_stamp in cache.cache.keys() {
        if stamp.large >= MAX_CACHE_TICKS_AMNT
            && recorded_stamp < &(stamp.large - MAX_CACHE_TICKS_AMNT)
        {
            to_remove.push(*recorded_stamp);
        }
    }
    for i in to_remove {
        cache.cache.remove(&i);
    }
}

/// Rigid body spawning.
pub fn build_rigid_bodies<T: RigidBodyBuilder<NoData> + 'static>(
    mut spawn_events: EventReader<SpawnEntity<T>>,
    mut commands: Commands,
    mut rigidbodies: ResMut<RigidBodies>,
    app_mode: Res<Mode>,
    mut new: ResMut<NewlySpawnedRigidbodies>,
    stamp: Res<TickRateStamp>,
) {
    for spawn_event in spawn_events.read() {
        let rigidbody_bundle = spawn_event
            .entity_type
            .get_bundle(&spawn_event.spawn_data, NoData);

        match new.cache.get_mut(&stamp.large) {
            Some(l) => {
                l.insert(
                    spawn_event.spawn_data.entity,
                    PhysicsSpawn {
                        translation: spawn_event.spawn_data.entity_transform.translation,
                        rotation: spawn_event.spawn_data.entity_transform.rotation,
                    },
                );
            }
            None => {
                let mut map = HashMap::new();
                map.insert(
                    spawn_event.spawn_data.entity,
                    PhysicsSpawn {
                        translation: spawn_event.spawn_data.entity_transform.translation,
                        rotation: spawn_event.spawn_data.entity_transform.rotation,
                    },
                );
                new.cache.insert(stamp.large, map);
            }
        }

        rigidbody_builder(
            &mut commands,
            RigidBodyBuildData {
                rigidbody_dynamic: rigidbody_bundle.rigidbody_dynamic,
                rigid_transform: spawn_event.spawn_data.entity_transform,
                entity_is_stored_item: spawn_event.spawn_data.holder_entity_option.is_some(),
                collider: rigidbody_bundle.collider,
                friction: rigidbody_bundle.collider_friction,
                collision_events: rigidbody_bundle.collision_events,
                locked_axes: rigidbody_bundle.locked_axes,
                external_force: rigidbody_bundle.external_force,
                mesh_offset: rigidbody_bundle.mesh_offset,
                ..Default::default()
            },
            spawn_event.spawn_data.entity,
            spawn_event.spawn_data.showcase_data_option.is_some(),
            &mut rigidbodies,
            &app_mode,
        );
    }
}
