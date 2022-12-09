use bevy::math::Mat4;
use bevy::math::Quat;
use bevy::math::Vec3;
use bevy::prelude::Commands;
use bevy::prelude::EventReader;
use bevy::prelude::EventWriter;
use bevy::prelude::Transform;
use bevy_rapier3d::prelude::{CoefficientCombineRule, Collider, Friction};
use combat::attack::DEFAULT_INVENTORY_ITEM_DAMAGE;
use entity::entity_data::RawSpawnEvent;
use entity::examine::Examinable;
use entity::examine::RichName;
use entity::health::DamageFlag;
use entity::spawn::BaseEntityBundle;
use entity::spawn::BaseEntitySummonable;
use entity::spawn::DefaultSpawnEvent;
use entity::spawn::NoData;
use entity::spawn::SpawnData;
use entity::spawn::SpawnEvent;
use inventory_api::core::SlotType;
use inventory_item::combat::CombatAttackAnimation;
use inventory_item::combat::DamageModel;
use inventory_item::combat::MeleeCombat;
use inventory_item::combat::ProjectileCombat;
use inventory_item::item::CombatStandardAnimation;
use inventory_item::item::InventoryItem;
use inventory_item::spawn::InventoryItemBundle;
use inventory_item::spawn::InventoryItemSummonable;
use rigid_body::rigid_body::STANDARD_BODY_FRICTION;
use rigid_body::spawn::RigidBodyBundle;
use rigid_body::spawn::RigidBodySummonable;
use std::collections::BTreeMap;
use text_api::core::Color;

#[cfg(feature = "server")]
pub fn get_default_transform() -> Transform {
    Transform::from_matrix(Mat4::from_scale_rotation_translation(
        Vec3::new(1., 1., 1.),
        Quat::from_axis_angle(Vec3::new(-0.00000035355248, 0.707105, 0.7071085), 3.1415951),
        Vec3::new(0., 0.116, 0.),
    ))
}

#[cfg(feature = "server")]
impl BaseEntitySummonable<NoData> for PistolL1Summoner {
    fn get_bundle(&self, _spawn_data: &SpawnData, _entity_data: NoData) -> BaseEntityBundle {
        let mut examine_map = BTreeMap::new();
        examine_map.insert(
            0,
            "A standard issue laser pistol. It is a lethal weapon.".to_string(),
        );

        BaseEntityBundle {
            default_transform: get_default_transform(),
            examinable: Examinable {
                assigned_texts: examine_map,
                name: RichName {
                    name: "laser pistol".to_string(),
                    n: false,
                    ..Default::default()
                },
                ..Default::default()
            },
            entity_name: PISTOL_L1_ENTITY_NAME.to_string(),

            ..Default::default()
        }
    }
}
use std::collections::HashMap;

#[cfg(feature = "server")]
pub const PISTOL_L1_PROJECTILE_RANGE: f32 = 50.;

#[cfg(feature = "server")]
impl InventoryItemSummonable for PistolL1Summoner {
    fn get_bundle(&self, spawn_data: &SpawnData) -> InventoryItemBundle {
        let mut attachment_transforms = HashMap::new();

        attachment_transforms.insert(
            "left_hand".to_string(),
            Transform::from_matrix(Mat4::from_scale_rotation_translation(
                Vec3::new(0.5, 0.5, 0.5),
                Quat::from_axis_angle(Vec3::new(-0.5695359, -0.7159382, 0.4038085), 2.4144572),
                Vec3::new(-0.031, 0.033, 0.011),
            )),
        );

        attachment_transforms.insert(
            "right_hand".to_string(),
            Transform::from_matrix(Mat4::from_scale_rotation_translation(
                Vec3::new(0.5, 0.5, 0.5),
                Quat::from_xyzw(0.611671, 0.396847, 0.530651, 0.432181),
                Vec3::new(0.077, -0.067, -0.045),
            )),
        );

        attachment_transforms.insert(
            "holster".to_string(),
            Transform::from_matrix(Mat4::from_scale_rotation_translation(
                Vec3::new(0.5, 0.5, 0.5),
                Quat::from_axis_angle(Vec3::new(0.004467, 0.0995011, -0.9950274), 3.0523109),
                Vec3::new(0., 0.132, 0.05),
            )),
        );

        let mut melee_damage_flags = HashMap::new();
        melee_damage_flags.insert(0, DamageFlag::SoftDamage);

        let mut projectile_damage_flags = HashMap::new();
        projectile_damage_flags.insert(0, DamageFlag::WeakLethalLaser);

        InventoryItemBundle {
            inventory_item: InventoryItem {
                in_inventory_of_entity: spawn_data.holder_entity_option,
                attachment_transforms: attachment_transforms,
                drop_transform: get_default_transform(),
                slot_type: SlotType::Holster,
                combat_standard_animation: CombatStandardAnimation::PistolStance,
                ..Default::default()
            },
            melee_combat: MeleeCombat {
                combat_melee_damage_model: DamageModel {
                    brute: DEFAULT_INVENTORY_ITEM_DAMAGE,
                    damage_flags: melee_damage_flags,
                    ..Default::default()
                },
                combat_attack_animation: CombatAttackAnimation::PistolShot,
                ..Default::default()
            },
            projectile_combat_option: Some(ProjectileCombat {
                combat_projectile_damage_model: DamageModel {
                    burn: 15.,
                    damage_flags: projectile_damage_flags,
                    ..Default::default()
                },
                laser_color: Color {
                    r: 1.,
                    g: 0.,
                    b: 0.,
                    a: 1.,
                },
                laser_height: 3.,
                laser_radius: 0.025,
                laser_range: PISTOL_L1_PROJECTILE_RANGE,
                ..Default::default()
            }),
        }
    }
}

#[cfg(feature = "server")]
impl RigidBodySummonable<NoData> for PistolL1Summoner {
    fn get_bundle(&self, _spawn_data: &SpawnData, _entity_data: NoData) -> RigidBodyBundle {
        let mut friction = Friction::coefficient(STANDARD_BODY_FRICTION);
        friction.combine_rule = CoefficientCombineRule::Multiply;

        RigidBodyBundle {
            collider: Collider::cuboid(0.047, 0.219, 0.199),
            collider_transform: Transform::from_translation(Vec3::new(0., 0.087, 0.)),
            collider_friction: friction,

            ..Default::default()
        }
    }
}

use crate::pistol_l1::PISTOL_L1_ENTITY_NAME;

use super::pistol_l1::PistolL1;

#[cfg(feature = "server")]
pub struct PistolL1Summoner;

#[cfg(feature = "server")]
pub fn summon_pistol_l1<T: Send + Sync + 'static>(
    mut commands: Commands,
    mut spawn_events: EventReader<SpawnEvent<T>>,
) {
    for spawn_event in spawn_events.iter() {
        commands
            .entity(spawn_event.spawn_data.entity)
            .insert(PistolL1);
    }
}

#[cfg(feature = "server")]
pub fn summon_raw_pistol_l1(
    mut spawn_events: EventReader<RawSpawnEvent>,
    mut summon_computer: EventWriter<SpawnEvent<PistolL1Summoner>>,
    mut commands: Commands,
) {
    for spawn_event in spawn_events.iter() {
        if spawn_event.raw_entity.entity_type != PISTOL_L1_ENTITY_NAME {
            continue;
        }

        let mut entity_transform = Transform::from_translation(spawn_event.raw_entity.translation);
        entity_transform.rotation = spawn_event.raw_entity.rotation;
        entity_transform.scale = spawn_event.raw_entity.scale;

        summon_computer.send(SpawnEvent {
            spawn_data: SpawnData {
                entity_transform: entity_transform,
                default_map_spawn: true,
                entity_name: spawn_event.raw_entity.entity_type.clone(),
                entity: commands.spawn(()).id(),
                raw_entity_option: Some(spawn_event.raw_entity.clone()),
                ..Default::default()
            },
            summoner: PistolL1Summoner,
        });
    }
}

#[cfg(feature = "server")]
pub fn default_summon_pistol_l1(
    mut default_spawner: EventReader<DefaultSpawnEvent>,
    mut spawner: EventWriter<SpawnEvent<PistolL1Summoner>>,
) {
    for spawn_event in default_spawner.iter() {
        if spawn_event.spawn_data.entity_name != PISTOL_L1_ENTITY_NAME {
            continue;
        }
        spawner.send(SpawnEvent {
            spawn_data: spawn_event.spawn_data.clone(),
            summoner: PistolL1Summoner,
        });
    }
}
