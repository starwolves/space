use std::collections::BTreeMap;

use super::resources::Airlock;
use bevy::{
    math::Vec3,
    prelude::{warn, Commands, EventReader, EventWriter, Transform},
};
use bevy_rapier3d::prelude::{CoefficientCombineRule, Collider, Friction};
use const_format::concatcp;
use entity::{
    entity_data::{EntityGroup, RawSpawnEvent},
    examine::{Examinable, RichName},
    health::Health,
    meta::SF_CONTENT_PREFIX,
    spawn::{
        BaseEntityBuildable, BaseEntityBundle, DefaultSpawnEvent, EntityBuildData, NoData,
        SpawnEntity,
    },
};
use pawn::pawn::ShipAuthorizationEnum;
use physics::spawn::{RigidBodyBuildable, RigidBodyBundle};
use text_api::core::{FURTHER_ITALIC_FONT, HEALTHY_COLOR};

#[cfg(feature = "server")]
pub fn get_default_transform() -> Transform {
    Transform::IDENTITY
}

#[cfg(feature = "server")]
impl BaseEntityBuildable<NoData> for AirlockBuilder {
    fn get_bundle(&self, spawn_data: &EntityBuildData, _entity_data: NoData) -> BaseEntityBundle {
        let description;
        let sub_name;

        if spawn_data.entity_name == SECURITY_AIRLOCK_ENTITY_NAME {
            sub_name = "security";
            description = "An air lock with ".to_string()
                + "security"
                + " department colors. It will only grant access to security personnel.";
        } else if spawn_data.entity_name == BRIDGE_AIRLOCK_ENTITY_NAME {
            sub_name = "bridge";
            description = "An air lock with ".to_string()
                + "bridge"
                + " department colors. It will only grant access to high ranked personnel.";
        } else if spawn_data.entity_name == GOVERNMENT_AIRLOCK_ENTITY_NAME {
            sub_name = "government";

            description = "An air lock with ".to_string()
                + "government"
                + " department colors. It will only grant access to a select few.";
        } else if spawn_data.entity_name == VACUUM_AIRLOCK_ENTITY_NAME {
            sub_name = "vacuum";
            description = "An air lock with ".to_string()
                + "danger markings"
                + ". On the other side is nothing but space.";
        } else {
            warn!("Unrecognized airlock sub-type {}", spawn_data.entity_name);
            sub_name = "ERR";
            description = "ERR ".to_string();
        }

        let mut examine_map = BTreeMap::new();
        examine_map.insert(0, description);
        examine_map.insert(
            1,
            "[font=".to_string()
                + FURTHER_ITALIC_FONT
                + "][color="
                + HEALTHY_COLOR
                + "]It is fully operational.[/color][/font]",
        );

        BaseEntityBundle {
            default_transform: get_default_transform(),
            examinable: Examinable {
                name: RichName {
                    name: sub_name.to_string() + " airlock",
                    n: false,
                    ..Default::default()
                },
                assigned_texts: examine_map,
                ..Default::default()
            },
            entity_name: spawn_data.entity_name.to_string(),
            entity_group: EntityGroup::AirLock,
            health: Health {
                is_combat_obstacle: true,
                is_reach_obstacle: true,
                ..Default::default()
            },
            default_map_spawn: spawn_data.default_map_spawn,
        }
    }
}

#[cfg(feature = "server")]
pub const DEFAULT_AIRLOCK_Y: f32 = 1.;

#[cfg(feature = "server")]
impl RigidBodyBuildable<NoData> for AirlockBuilder {
    fn get_bundle(&self, _spawn_data: &EntityBuildData, _entity_data: NoData) -> RigidBodyBundle {
        let mut friction = Friction::coefficient(0.);
        friction.combine_rule = CoefficientCombineRule::Multiply;

        RigidBodyBundle {
            collider: Collider::cuboid(1., 1., 0.2),
            collider_transform: Transform::from_translation(Vec3::new(0., DEFAULT_AIRLOCK_Y, 0.)),
            collider_friction: friction,
            rigidbody_dynamic: false,
            collision_events: true,
        }
    }
}

#[cfg(feature = "server")]
pub struct AirlockBuilder;

#[cfg(feature = "server")]
pub fn build_airlocks<T: Send + Sync + 'static>(
    mut commands: Commands,
    mut airlock_spawns: EventReader<SpawnEntity<T>>,
) {
    for spawn_event in airlock_spawns.iter() {
        commands
            .entity(spawn_event.spawn_data.entity)
            .insert(Airlock {
                access_permissions: vec![ShipAuthorizationEnum::Security],
                ..Default::default()
            });
    }
}

pub const SECURITY_AIRLOCK_ENTITY_NAME: &str = concatcp!(SF_CONTENT_PREFIX, "securityAirLock1");
pub const BRIDGE_AIRLOCK_ENTITY_NAME: &str = concatcp!(SF_CONTENT_PREFIX, "bridgeAirLock");
pub const GOVERNMENT_AIRLOCK_ENTITY_NAME: &str = concatcp!(SF_CONTENT_PREFIX, "governmentAirLock");
pub const VACUUM_AIRLOCK_ENTITY_NAME: &str = concatcp!(SF_CONTENT_PREFIX, "vacuumAirLock");

#[cfg(feature = "server")]
pub fn default_build_airlocks(
    mut default_spawner: EventReader<DefaultSpawnEvent>,
    mut spawner: EventWriter<SpawnEntity<AirlockBuilder>>,
) {
    for spawn_event in default_spawner.iter() {
        if spawn_event.spawn_data.entity_name != SECURITY_AIRLOCK_ENTITY_NAME
            || spawn_event.spawn_data.entity_name != BRIDGE_AIRLOCK_ENTITY_NAME
            || spawn_event.spawn_data.entity_name != GOVERNMENT_AIRLOCK_ENTITY_NAME
            || spawn_event.spawn_data.entity_name != VACUUM_AIRLOCK_ENTITY_NAME
        {
            continue;
        }

        spawner.send(SpawnEntity {
            spawn_data: spawn_event.spawn_data.clone(),
            builder: AirlockBuilder,
        });
    }
}

#[cfg(feature = "server")]
pub fn build_raw_airlocks(
    mut spawn_events: EventReader<RawSpawnEvent>,
    mut build_airlock: EventWriter<SpawnEntity<AirlockBuilder>>,
    mut commands: Commands,
) {
    for spawn_event in spawn_events.iter() {
        if spawn_event.raw_entity.entity_type != SECURITY_AIRLOCK_ENTITY_NAME
            && spawn_event.raw_entity.entity_type != BRIDGE_AIRLOCK_ENTITY_NAME
            && spawn_event.raw_entity.entity_type != GOVERNMENT_AIRLOCK_ENTITY_NAME
            && spawn_event.raw_entity.entity_type != VACUUM_AIRLOCK_ENTITY_NAME
        {
            continue;
        }

        let mut entity_transform = Transform::from_translation(spawn_event.raw_entity.translation);
        entity_transform.rotation = spawn_event.raw_entity.rotation;
        entity_transform.scale = spawn_event.raw_entity.scale;

        build_airlock.send(SpawnEntity {
            spawn_data: EntityBuildData {
                entity_transform: entity_transform,
                default_map_spawn: true,
                entity_name: spawn_event.raw_entity.entity_type.clone(),
                entity: commands.spawn(()).id(),
                raw_entity_option: Some(spawn_event.raw_entity.clone()),
                ..Default::default()
            },
            builder: AirlockBuilder,
        });
    }
}
