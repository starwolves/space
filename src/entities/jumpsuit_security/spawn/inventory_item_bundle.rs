use std::collections::HashMap;

use bevy_ecs::entity::Entity;
use bevy_math::{Mat4, Quat, Vec3};
use bevy_transform::prelude::Transform;

use crate::core::{
    health::components::{DamageFlag, DamageModel},
    inventory::components::SlotType,
    inventory_item::{components::InventoryItem, spawn::InventoryItemBundle},
};

pub fn inventory_item_bundle(
    holder_option: Option<Entity>,
    default_transform: Transform,
) -> InventoryItemBundle {
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
            in_inventory_of_entity: holder_option,
            attachment_transforms: attachment_transforms,
            drop_transform: default_transform,
            slot_type: SlotType::Jumpsuit,
            combat_melee_damage_model: DamageModel {
                brute: 5.,
                damage_flags: melee_damage_flags,
                ..Default::default()
            },
            throw_force_factor: 2.,
            ..Default::default()
        },
    }
}
