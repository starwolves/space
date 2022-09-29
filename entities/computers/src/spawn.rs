use api::{converters::string_transform_to_transform, data::NoData};
use bevy::{
    math::{Mat4, Quat, Vec3},
    prelude::{warn, Commands, EventReader, EventWriter, Transform},
};
use bevy_rapier3d::prelude::{CoefficientCombineRule, Collider, Friction};
use entity::{
    entity_data::RawSpawnEvent,
    spawn::{
        BaseEntityBundle, BaseEntitySummonable, DefaultSpawnEvent, ExportProperty, SpawnData,
        SpawnEvent,
    },
};
use examinable::examine::{Examinable, RichName};
use health::core::Health;
use rigid_body::{
    rigid_body::STANDARD_BODY_FRICTION,
    spawn::{RigidBodyBundle, RigidBodySummonable},
};
use std::collections::BTreeMap;

pub fn get_default_transform() -> Transform {
    Transform::from_matrix(Mat4::from_scale_rotation_translation(
        Vec3::new(1., 1., 1.),
        Quat::from_axis_angle(Vec3::new(-0.0394818427, 0.00003351599, 1.), 3.124470974),
        Vec3::new(0., 0.355, 0.),
    ))
}

impl BaseEntitySummonable<NoData> for ComputerSummoner {
    fn get_bundle(&self, _spawn_data: &SpawnData, _entity_data: NoData) -> BaseEntityBundle {
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
            entity_name: BRIDGE_COMPUTER_ENTITY_NAME.to_string(),
            health: Health {
                is_combat_obstacle: true,
                is_reach_obstacle: true,
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

impl RigidBodySummonable<NoData> for ComputerSummoner {
    fn get_bundle(&self, _spawn_data: &SpawnData, _entity_data: NoData) -> RigidBodyBundle {
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

pub struct ComputerSummoner {
    pub computer_type: String,
}

impl ComputerSummonable for ComputerSummoner {
    fn get_computer_type(&self) -> String {
        self.computer_type.clone()
    }
}

pub trait ComputerSummonable {
    fn get_computer_type(&self) -> String;
}

pub fn summon_computer<T: ComputerSummonable + Send + Sync + 'static>(
    mut commands: Commands,
    mut spawn_events: EventReader<SpawnEvent<T>>,
) {
    for spawn_event in spawn_events.iter() {
        commands
            .entity(spawn_event.spawn_data.entity)
            .insert(Computer {
                computer_type: spawn_event.summoner.get_computer_type().clone(),
            });
    }
}

pub const BRIDGE_COMPUTER_ENTITY_NAME: &str = "bridgeComputer";

pub fn summon_raw_computer(
    mut spawn_events: EventReader<RawSpawnEvent>,
    mut summon_computer: EventWriter<SpawnEvent<ComputerSummoner>>,
    mut commands: Commands,
) {
    for spawn_event in spawn_events.iter() {
        if spawn_event.raw_entity.entity_type != BRIDGE_COMPUTER_ENTITY_NAME {
            continue;
        }

        let entity_transform = string_transform_to_transform(&spawn_event.raw_entity.transform);

        let data_result: Result<Vec<ExportProperty>, _> =
            serde_json::from_str(&spawn_event.raw_entity.data);

        let mut computer_name = "".to_string();

        match data_result {
            Ok(ds) => {
                for d in ds {
                    if d.key == "computerType" {
                        if d.value_type == 4 {
                            computer_name = d.value;
                        } else {
                            warn!("Entity from entities.json had unknown type!");
                            continue;
                        }
                    }
                }
            }
            Err(_rr) => {
                warn!("Invalid json!");
                warn!("{}", spawn_event.raw_entity.data);
                continue;
            }
        }

        summon_computer.send(SpawnEvent {
            spawn_data: SpawnData {
                entity_transform: entity_transform,
                default_map_spawn: true,
                entity_name: spawn_event.raw_entity.entity_type.clone(),
                entity: commands.spawn().id(),
                raw_entity_option: Some(spawn_event.raw_entity.clone()),
                ..Default::default()
            },
            summoner: ComputerSummoner {
                computer_type: computer_name,
            },
        });
    }
}

use super::computer::Computer;

pub fn default_summon_computer(
    mut default_spawner: EventReader<DefaultSpawnEvent>,
    mut spawner: EventWriter<SpawnEvent<ComputerSummoner>>,
) {
    for spawn_event in default_spawner.iter() {
        if spawn_event.spawn_data.entity_name != BRIDGE_COMPUTER_ENTITY_NAME {
            continue;
        }
        spawner.send(SpawnEvent {
            spawn_data: spawn_event.spawn_data.clone(),
            summoner: ComputerSummoner {
                computer_type: "".to_string(),
            },
        });
    }
}
