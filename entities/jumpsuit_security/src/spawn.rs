use bevy::math::Mat4;
use bevy::math::Quat;
use bevy::math::Vec3;
use bevy::prelude::Commands;
use bevy::prelude::EventReader;
use bevy::prelude::EventWriter;
use bevy::prelude::Transform;
use bevy_rapier3d::prelude::{CoefficientCombineRule, Collider, Friction};
use entity::entity_data::RawSpawnEvent;
use entity::examine::Examinable;
use entity::examine::RichName;
use entity::health::DamageFlag;
use entity::spawn::BaseEntityBuilder;
use entity::spawn::BaseEntityBundle;
use entity::spawn::DefaultSpawnEvent;
use entity::spawn::EntityBuildData;
use entity::spawn::EntityType;
use entity::spawn::NoData;
use entity::spawn::SpawnEntity;
use inventory::combat::DamageModel;
use inventory::combat::MeleeCombat;
use inventory::inventory::SlotType;
use inventory::item::InventoryItem;
use inventory::spawn_item::InventoryItemBuilder;
use inventory::spawn_item::InventoryItemBundle;
use physics::rigid_body::STANDARD_BODY_FRICTION;
use physics::spawn::RigidBodyBuilder;
use physics::spawn::RigidBodyBundle;
use std::collections::BTreeMap;
use std::collections::HashMap;

use crate::jumpsuit::JUMPSUIT_SECURITY_ENTITY_NAME;

use super::jumpsuit::Jumpsuit;

#[cfg(feature = "server")]
pub fn get_default_transform() -> Transform {
    Transform::from_matrix(Mat4::from_scale_rotation_translation(
        Vec3::new(1., 1., 1.),
        Quat::from_axis_angle(Vec3::new(-0.00000035355248, 0.707105, 0.7071085), 3.1415951),
        Vec3::new(0., 0.116, 0.),
    ))
}

#[cfg(feature = "server")]
impl BaseEntityBuilder<NoData> for JumpsuitType {
    fn get_bundle(&self, _spawn_data: &EntityBuildData, _entity_data: NoData) -> BaseEntityBundle {
        let mut examine_map = BTreeMap::new();
        examine_map.insert(
            0,
            "A standard issue security jumpsuit used by Security Officers.".to_string(),
        );

        BaseEntityBundle {
            default_transform: get_default_transform(),
            examinable: Examinable {
                assigned_texts: examine_map,
                name: RichName {
                    name: "security jumpsuit".to_string(),
                    n: false,
                    ..Default::default()
                },
                ..Default::default()
            },
            entity_type: Box::new(JumpsuitType::new()),

            ..Default::default()
        }
    }
}

#[cfg(feature = "server")]
impl InventoryItemBuilder for JumpsuitType {
    fn get_bundle(&self, spawn_data: &EntityBuildData) -> InventoryItemBundle {
        let mut attachment_transforms = HashMap::new();

        let left_hand_rotation = Vec3::new(-0.324509068, -1.52304412, 2.79253);
        let left_hand_rotation_length = left_hand_rotation.length();

        attachment_transforms.insert(
            "left_hand".to_string(),
            Transform::from_matrix(Mat4::from_scale_rotation_translation(
                Vec3::new(0.5, 0.5, 0.5),
                Quat::from_axis_angle(left_hand_rotation.normalize(), left_hand_rotation_length),
                Vec3::new(0.003, 0.069, 0.012),
            )),
        );

        let right_hand_rotation = Vec3::new(-0.202877072, -0.762290004, -0.190973927);
        let right_hand_rotation_length = right_hand_rotation.length();

        attachment_transforms.insert(
            "right_hand".to_string(),
            Transform::from_matrix(Mat4::from_scale_rotation_translation(
                Vec3::new(0.5, 0.5, 0.5),
                Quat::from_axis_angle(right_hand_rotation.normalize(), right_hand_rotation_length),
                Vec3::new(0.026, -0.008, 0.004),
            )),
        );

        let mut melee_damage_flags = HashMap::new();
        melee_damage_flags.insert(0, DamageFlag::SoftDamage);

        InventoryItemBundle {
            inventory_item: InventoryItem {
                is_attached_when_worn: false,
                in_inventory_of_entity: spawn_data.holder_entity_option,
                attachment_transforms: attachment_transforms,
                drop_transform: get_default_transform(),
                slot_type: SlotType::Jumpsuit,
                throw_force_factor: 2.,
                ..Default::default()
            },
            melee_combat: MeleeCombat {
                combat_melee_damage_model: DamageModel {
                    brute: 5.,
                    damage_flags: melee_damage_flags,
                    ..Default::default()
                },
                ..Default::default()
            },
            projectile_combat_option: None,
        }
    }
}

#[cfg(feature = "server")]
impl RigidBodyBuilder<NoData> for JumpsuitType {
    fn get_bundle(&self, _spawn_data: &EntityBuildData, _entity_data: NoData) -> RigidBodyBundle {
        let mut friction = Friction::coefficient(STANDARD_BODY_FRICTION);
        friction.combine_rule = CoefficientCombineRule::Multiply;

        RigidBodyBundle {
            collider: Collider::cuboid(0.269, 0.377, 0.098),
            collider_transform: Transform::from_translation(Vec3::new(0., -0.021, -0.011)),
            collider_friction: friction,

            ..Default::default()
        }
    }
}

#[cfg(feature = "server")]
pub struct JumpsuitType;
impl EntityType for JumpsuitType {
    fn to_string(&self) -> String {
        JUMPSUIT_SECURITY_ENTITY_NAME.to_owned()
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        JumpsuitType
    }
}
#[cfg(feature = "server")]
pub fn build_jumpsuits<T: Send + Sync + 'static>(
    mut commands: Commands,
    mut spawn_events: EventReader<SpawnEntity<T>>,
) {
    for spawn_event in spawn_events.iter() {
        commands
            .entity(spawn_event.spawn_data.entity)
            .insert(Jumpsuit);
    }
}

#[cfg(feature = "server")]
pub fn build_raw_jumpsuits(
    mut spawn_events: EventReader<RawSpawnEvent>,
    mut builder_computer: EventWriter<SpawnEntity<JumpsuitType>>,
    mut commands: Commands,
) {
    for spawn_event in spawn_events.iter() {
        if spawn_event.raw_entity.entity_type != JUMPSUIT_SECURITY_ENTITY_NAME {
            continue;
        }

        let mut entity_transform = Transform::from_translation(spawn_event.raw_entity.translation);
        entity_transform.rotation = spawn_event.raw_entity.rotation;
        entity_transform.scale = spawn_event.raw_entity.scale;
        builder_computer.send(SpawnEntity {
            spawn_data: EntityBuildData {
                entity_transform: entity_transform,
                default_map_spawn: true,
                entity_type: spawn_event.raw_entity.entity_type.clone(),
                entity: commands.spawn(()).id(),
                raw_entity_option: Some(spawn_event.raw_entity.clone()),
                ..Default::default()
            },
            builder: JumpsuitType,
        });
    }
}

#[cfg(feature = "server")]
pub fn default_build_jumpsuits(
    mut default_spawner: EventReader<DefaultSpawnEvent>,
    mut spawner: EventWriter<SpawnEntity<JumpsuitType>>,
) {
    for spawn_event in default_spawner.iter() {
        if spawn_event.spawn_data.entity_type != JUMPSUIT_SECURITY_ENTITY_NAME {
            continue;
        }

        spawner.send(SpawnEntity {
            spawn_data: spawn_event.spawn_data.clone(),
            builder: JumpsuitType,
        });
    }
}
