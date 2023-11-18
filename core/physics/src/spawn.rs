use crate::{
    entity::{RigidBodies, RigidBodyLink, SFRigidBody},
    physics::{get_bit_masks, ColliderGroup},
    rigid_body::RigidBodyData,
};
use bevy::{
    prelude::{Commands, Entity, EventReader, Res, ResMut, Transform},
    transform::TransformBundle,
};
use bevy_xpbd_3d::prelude::{
    Collider, CollisionLayers, ExternalForce, Friction, LinearVelocity, LockedAxes, RigidBody,
    Sleeping,
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
    pub sleeping: Sleeping,
    pub entity_is_stored_item: bool,
    pub collider: Collider,
    pub collider_friction: Friction,
    pub collider_collision_layers: CollisionLayers,
    pub collision_events: bool,
    pub mesh_offset: Transform,
    pub locked_axes: LockedAxes,
}

impl Default for RigidBodyBuildData {
    fn default() -> Self {
        let masks = get_bit_masks(ColliderGroup::Standard);
        Self {
            rigidbody_dynamic: false,
            rigid_transform: Transform::default(),
            external_force: ExternalForce::default(),
            linear_velocity: LinearVelocity::default(),
            sleeping: Sleeping::default(),
            entity_is_stored_item: false,
            collider: Collider::cuboid(0.2, 0.2, 0.2),
            collider_friction: Friction::default(),
            collider_collision_layers: CollisionLayers::from_bits(masks.0, masks.1),
            collision_events: false,
            locked_axes: LockedAxes::new(),
            mesh_offset: Transform::default(),
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
    let mut builder = commands.spawn((
        t.clone(),
        rigidbody,
        rigidbody_spawn_data.external_force,
        rigidbody_spawn_data.linear_velocity,
        RigidBodyData {
            dynamic_friction: rigidbody_spawn_data.collider_friction.dynamic_coefficient,
            static_friction: rigidbody_spawn_data.collider_friction.static_coefficient,
            friction_combine_rule: rigidbody_spawn_data.collider_friction.combine_rule,
        },
        rigidbody_spawn_data.collider,
        rigidbody_spawn_data.collider_friction,
        masks,
        SFRigidBody,
        rigidbody_spawn_data.locked_axes,
    ));

    let rigid_entity = builder.id();

    if rigidbody_spawn_data.entity_is_stored_item {
        builder.insert(Sleeping);
    }

    let mut builder = commands.entity(entity);

    if !(is_server() || matches!(**app_mode, Mode::Correction)) {
        t.local.translation += rigidbody_spawn_data.mesh_offset.translation;
        t.local.scale = rigidbody_spawn_data.mesh_offset.scale;
        t.local.rotation *= rigidbody_spawn_data.mesh_offset.rotation;
    }
    builder.insert((
        t.clone(),
        RigidBodyLink {
            offset: rigidbody_spawn_data.mesh_offset,
            target_transform: t.local.clone(),
            origin_transfom: t.local,
            ..Default::default()
        },
    ));
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

    let mut builder = commands.entity(rigid_entity);
    if !rigidbody_enabled {
        builder.insert(Sleeping);
    }

    rigidbodies.link_entity(&entity, &rigid_entity)
}

pub trait RigidBodyBuilder<Y>: Send + Sync {
    fn get_bundle(&self, spawn_data: &EntityBuildData, entity_data_option: Y) -> RigidBodyBundle;
}
use entity::spawn::{NoData, SpawnEntity};
use resources::modes::{is_server, Mode};

/// Rigid body spawning.
pub fn build_rigid_bodies<T: RigidBodyBuilder<NoData> + 'static>(
    mut spawn_events: EventReader<SpawnEntity<T>>,
    mut commands: Commands,
    mut rigidbodies: ResMut<RigidBodies>,
    app_mode: Res<Mode>,
) {
    for spawn_event in spawn_events.read() {
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
                collider_friction: rigidbody_bundle.collider_friction,
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
