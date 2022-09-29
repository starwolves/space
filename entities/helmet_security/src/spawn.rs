use std::collections::BTreeMap;

pub fn get_default_transform() -> Transform {
    Transform::from_matrix(Mat4::from_scale_rotation_translation(
        Vec3::new(1., 1., 1.),
        Quat::from_axis_angle(Vec3::new(-0.0394818427, 0.00003351599, 1.), 3.124470974),
        Vec3::new(0., 0.355, 0.),
    ))
}

impl BaseEntitySummonable<NoData> for HelmetSummoner {
    fn get_bundle(&self, _spawn_data: &SpawnData, _entity_data: NoData) -> BaseEntityBundle {
        let mut examine_map = BTreeMap::new();
        examine_map.insert(
            0,
            "A standard issue helmet used by Security Officers.".to_string(),
        );
        BaseEntityBundle {
            default_transform: get_default_transform(),
            examinable: Examinable {
                assigned_texts: examine_map,
                name: RichName {
                    name: "security helmet".to_string(),
                    n: false,
                    ..Default::default()
                },
                ..Default::default()
            },
            entity_name: HELMET_SECURITY_ENTITY_NAME.to_string(),
            ..Default::default()
        }
    }
}
use std::collections::HashMap;

impl InventoryItemSummonable for HelmetSummoner {
    fn get_bundle(&self, spawn_data: &SpawnData) -> InventoryItemBundle {
        let mut attachment_transforms = HashMap::new();

        attachment_transforms.insert(
            "left_hand".to_string(),
            Transform::from_matrix(Mat4::from_scale_rotation_translation(
                Vec3::new(0.5, 0.5, 0.5),
                Quat::from_axis_angle(Vec3::new(1., 0., 0.), 3.111607897),
                Vec3::new(0., -0.003, -0.108),
            )),
        );

        let right_hand_rotation = Vec3::new(0.11473795, 0.775676679, 0.);
        let right_hand_rotation_length = right_hand_rotation.length();

        attachment_transforms.insert(
            "right_hand".to_string(),
            Transform::from_matrix(Mat4::from_scale_rotation_translation(
                Vec3::new(0.5, 0.5, 0.5),
                Quat::from_axis_angle(
                    Vec3::new(0.11473795, 0.775676679, 0.).normalize(),
                    right_hand_rotation_length,
                ),
                Vec3::new(0.064, -0.019, 0.065),
            )),
        );

        attachment_transforms.insert(
            "helmet".to_string(),
            Transform::from_matrix(Mat4::from_scale_rotation_translation(
                Vec3::new(0.5, 0.5, 0.5),
                Quat::from_axis_angle(Vec3::new(1., 0., 0.), -1.41617761),
                Vec3::new(0., 0.132, 0.05),
            )),
        );

        let mut melee_damage_flags = HashMap::new();
        melee_damage_flags.insert(0, DamageFlag::SoftDamage);

        InventoryItemBundle {
            inventory_item: InventoryItem {
                in_inventory_of_entity: spawn_data.holder_entity_option,
                attachment_transforms: attachment_transforms,
                drop_transform: get_default_transform(),
                slot_type: SlotType::Helmet,
                throw_force_factor: 2.,
                ..Default::default()
            },
            melee_combat: MeleeCombat {
                combat_melee_damage_model: DamageModel {
                    brute: DEFAULT_INVENTORY_ITEM_DAMAGE,
                    damage_flags: melee_damage_flags,
                    ..Default::default()
                },
                ..Default::default()
            },
            projectile_combat_option: None,
        }
    }
}
use api::combat::DamageFlag;
use api::combat::DamageModel;
use api::combat::MeleeCombat;
use api::combat::DEFAULT_INVENTORY_ITEM_DAMAGE;
use api::converters::string_transform_to_transform;
use api::data::NoData;
use api::inventory::SlotType;
use bevy::math::Mat4;
use bevy::math::Quat;
use bevy::math::Vec3;
use bevy::prelude::Commands;
use bevy::prelude::EventReader;
use bevy::prelude::EventWriter;
use bevy::prelude::Transform;
use bevy_rapier3d::prelude::{CoefficientCombineRule, Collider, Friction};
use entity::entity_data::RawSpawnEvent;
use entity::entity_data::HELMET_SECURITY_ENTITY_NAME;
use entity::spawn::BaseEntityBundle;
use entity::spawn::BaseEntitySummonable;
use entity::spawn::DefaultSpawnEvent;
use entity::spawn::SpawnData;
use entity::spawn::SpawnEvent;
use examinable::examine::Examinable;
use examinable::examine::RichName;
use inventory_item::item::InventoryItem;
use inventory_item::spawn::InventoryItemBundle;
use inventory_item::spawn::InventoryItemSummonable;
use rigid_body::rigid_body::STANDARD_BODY_FRICTION;
use rigid_body::spawn::RigidBodyBundle;
use rigid_body::spawn::RigidBodySummonable;

use super::helmet::Helmet;

impl RigidBodySummonable<NoData> for HelmetSummoner {
    fn get_bundle(&self, _spawn_data: &SpawnData, _entity_data: NoData) -> RigidBodyBundle {
        let mut friction = Friction::coefficient(STANDARD_BODY_FRICTION);
        friction.combine_rule = CoefficientCombineRule::Multiply;

        RigidBodyBundle {
            collider: Collider::cuboid(0.208, 0.277, 0.213),
            collider_transform: Transform::from_translation(Vec3::new(0., 0.011, -0.004)),
            collider_friction: friction,

            ..Default::default()
        }
    }
}

pub struct HelmetSummoner;

pub fn summon_helmet<T: Send + Sync + 'static>(
    mut commands: Commands,
    mut spawn_events: EventReader<SpawnEvent<T>>,
) {
    for spawn_event in spawn_events.iter() {
        commands
            .entity(spawn_event.spawn_data.entity)
            .insert(Helmet);
    }
}

pub fn summon_raw_helmet(
    mut spawn_events: EventReader<RawSpawnEvent>,
    mut summon_computer: EventWriter<SpawnEvent<HelmetSummoner>>,
    mut commands: Commands,
) {
    for spawn_event in spawn_events.iter() {
        if spawn_event.raw_entity.entity_type != HELMET_SECURITY_ENTITY_NAME {
            continue;
        }

        let entity_transform = string_transform_to_transform(&spawn_event.raw_entity.transform);

        summon_computer.send(SpawnEvent {
            spawn_data: SpawnData {
                entity_transform: entity_transform,
                default_map_spawn: true,
                entity_name: spawn_event.raw_entity.entity_type.clone(),
                entity: commands.spawn().id(),
                raw_entity_option: Some(spawn_event.raw_entity.clone()),
                ..Default::default()
            },
            summoner: HelmetSummoner,
        });
    }
}

pub fn default_summon_helmet_security(
    mut default_spawner: EventReader<DefaultSpawnEvent>,
    mut spawner: EventWriter<SpawnEvent<HelmetSummoner>>,
) {
    for spawn_event in default_spawner.iter() {
        if spawn_event.spawn_data.entity_name != HELMET_SECURITY_ENTITY_NAME {
            continue;
        }
        spawner.send(SpawnEvent {
            spawn_data: spawn_event.spawn_data.clone(),
            summoner: HelmetSummoner,
        });
    }
}
