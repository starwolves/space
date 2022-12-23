use bevy::{
    math::{Mat4, Quat, Vec3},
    prelude::{Commands, EventReader, EventWriter, Transform},
};
use bevy_rapier3d::prelude::{CoefficientCombineRule, Collider, Friction};
use entity::{
    entity_data::RawSpawnEvent,
    examine::{Examinable, RichName},
    health::Health,
    spawn::{
        BaseEntityBuilder, BaseEntityBundle, DefaultSpawnEvent, EntityBuildData, EntityType,
        NoData, SpawnEntity,
    },
};
use physics::{
    rigid_body::STANDARD_BODY_FRICTION,
    spawn::{RigidBodyBuilder, RigidBodyBundle},
};
use std::collections::BTreeMap;

#[cfg(any(feature = "server", feature = "client"))]
pub fn get_default_transform() -> Transform {
    Transform::from_matrix(Mat4::from_scale_rotation_translation(
        Vec3::new(1., 1., 1.),
        Quat::from_axis_angle(Vec3::new(-0.0394818427, 0.00003351599, 1.), 3.124470974),
        Vec3::new(0., 0.355, 0.),
    ))
}

#[cfg(any(feature = "server", feature = "client"))]
impl BaseEntityBuilder<NoData> for ComputerType {
    fn get_bundle(&self, _spawn_data: &EntityBuildData, _entity_data: NoData) -> BaseEntityBundle {
        let template_examine_text = "A computer used by bridge personnel.".to_string();
        let mut examine_map = BTreeMap::new();
        examine_map.insert(0, template_examine_text);

        BaseEntityBundle {
            default_transform: get_default_transform(),
            examinable: Examinable {
                assigned_texts: examine_map,
                name: RichName {
                    name: "bridge computer".to_string(),
                    n: false,
                    ..Default::default()
                },
                ..Default::default()
            },
            entity_type: BRIDGE_COMPUTER_ENTITY_NAME.to_string(),
            health: Health {
                is_combat_obstacle: true,
                is_reach_obstacle: true,
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

#[cfg(any(feature = "server", feature = "client"))]
impl RigidBodyBuilder<NoData> for ComputerType {
    fn get_bundle(&self, _spawn_data: &EntityBuildData, _entity_data: NoData) -> RigidBodyBundle {
        let mut friction = Friction::coefficient(STANDARD_BODY_FRICTION);
        friction.combine_rule = CoefficientCombineRule::Min;

        RigidBodyBundle {
            collider: Collider::cuboid(1., 0.7, 1.),
            collider_transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            collider_friction: friction,
            rigidbody_dynamic: false,
            collision_events: true,
        }
    }
}

#[cfg(any(feature = "server", feature = "client"))]
pub struct ComputerType;

impl EntityType for ComputerType {
    fn to_string() -> String {
        BRIDGE_COMPUTER_ENTITY_NAME.to_owned()
    }
}

#[cfg(any(feature = "server", feature = "client"))]
pub fn build_computers<T: Send + Sync + 'static>(
    mut commands: Commands,
    mut spawn_events: EventReader<SpawnEntity<T>>,
) {
    for spawn_event in spawn_events.iter() {
        commands
            .entity(spawn_event.spawn_data.entity)
            .insert(Computer);
    }
}
use const_format::concatcp;
use resources::content::SF_CONTENT_PREFIX;

#[cfg(any(feature = "server", feature = "client"))]
pub const BRIDGE_COMPUTER_ENTITY_NAME: &str = concatcp!(SF_CONTENT_PREFIX, "bridgeComputer");

#[cfg(any(feature = "server", feature = "client"))]
pub fn build_raw_computers(
    mut spawn_events: EventReader<RawSpawnEvent>,
    mut build_computer: EventWriter<SpawnEntity<ComputerType>>,
    mut commands: Commands,
) {
    for spawn_event in spawn_events.iter() {
        if spawn_event.raw_entity.entity_type != BRIDGE_COMPUTER_ENTITY_NAME {
            continue;
        }

        let mut entity_transform = Transform::from_translation(spawn_event.raw_entity.translation);
        entity_transform.rotation = spawn_event.raw_entity.rotation;
        entity_transform.scale = spawn_event.raw_entity.scale;

        build_computer.send(SpawnEntity {
            spawn_data: EntityBuildData {
                entity_transform: entity_transform,
                default_map_spawn: true,
                entity_type: spawn_event.raw_entity.entity_type.clone(),
                entity: commands.spawn(()).id(),
                raw_entity_option: Some(spawn_event.raw_entity.clone()),
                ..Default::default()
            },
            builder: ComputerType,
        });
    }
}

use super::computer::Computer;

#[cfg(any(feature = "server", feature = "client"))]
pub fn default_build_computers(
    mut default_spawner: EventReader<DefaultSpawnEvent>,
    mut spawner: EventWriter<SpawnEntity<ComputerType>>,
) {
    for spawn_event in default_spawner.iter() {
        if spawn_event.spawn_data.entity_type != BRIDGE_COMPUTER_ENTITY_NAME {
            continue;
        }
        spawner.send(SpawnEntity {
            spawn_data: spawn_event.spawn_data.clone(),
            builder: ComputerType,
        });
    }
}
