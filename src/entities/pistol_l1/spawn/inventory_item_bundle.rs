use std::collections::HashMap;

use bevy_math::{Mat4, Quat, Vec3};
use bevy_transform::prelude::Transform;

use crate::{
    core::{
        entity::resources::SpawnData,
        health::components::{DamageFlag, DamageModel},
        inventory::components::SlotType,
        inventory_item::{
            components::{
                CombatAttackAnimation, CombatSoundSet, CombatStandardAnimation, CombatType,
                InventoryItem, ProjectileType,
            },
            spawn::{InventoryItemBundle, InventoryItemSummonable},
        },
    },
    entities::pistol_l1::PistolL1Summoner,
};

use super::entity_bundle::get_default_transform;

pub const PISTOL_L1_PROJECTILE_RANGE: f32 = 50.;

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
                combat_attack_animation: CombatAttackAnimation::PistolShot,
                combat_type: CombatType::Projectile(ProjectileType::Laser(
                    (1., 0., 0., 1.),
                    3.,
                    0.025,
                    PISTOL_L1_PROJECTILE_RANGE,
                )),
                combat_melee_damage_model: DamageModel {
                    brute: 9.,
                    damage_flags: melee_damage_flags,
                    ..Default::default()
                },
                combat_projectile_damage_model: Some(DamageModel {
                    burn: 15.,
                    damage_flags: projectile_damage_flags,
                    ..Default::default()
                }),
                combat_standard_animation: CombatStandardAnimation::PistolStance,
                combat_projectile_sound_set: Some(CombatSoundSet::default_laser_projectiles()),
                combat_projectile_text_set: Some(InventoryItem::get_default_laser_words()),
                trigger_projectile_text_set: Some(InventoryItem::get_default_trigger_weapon_words()),
                ..Default::default()
            },
        }
    }
}
