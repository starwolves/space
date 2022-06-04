use std::collections::HashMap;

use bevy_math::{Mat4, Quat, Vec3};
use bevy_transform::prelude::Transform;

use crate::core::{
    entity::resources::SpawnData,
    health::components::{DamageFlag, DamageModel},
    inventory::components::SlotType,
    inventory_item::{
        components::InventoryItem,
        spawn::{InventoryItemBundle, InventoryItemSummonable},
    },
};

use super::{entity_bundle::get_default_transform, HelmetSummoner};

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
                in_inventory_of_entity: spawn_data.held_data_option,
                attachment_transforms: attachment_transforms,
                drop_transform: get_default_transform(),
                slot_type: SlotType::Helmet,
                combat_melee_damage_model: DamageModel {
                    brute: 9.,
                    damage_flags: melee_damage_flags,
                    ..Default::default()
                },
                throw_force_factor: 2.,
                ..Default::default()
            },
        }
    }
}
