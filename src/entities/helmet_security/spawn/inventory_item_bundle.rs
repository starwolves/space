use std::collections::HashMap;

use bevy_ecs::entity::Entity;
use bevy_math::{Mat4, Quat, Vec3};
use bevy_transform::prelude::Transform;

use crate::core::{
    health::components::{DamageFlag, DamageModel},
    inventory::components::SlotType,
    inventory_item::{components::InventoryItem, spawn::InventoryItemBundle},
};

pub fn inventory_item_bundle(holder_option: Option<Entity>) -> InventoryItemBundle {
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

    let default_transform = Transform::from_matrix(Mat4::from_scale_rotation_translation(
        Vec3::new(1., 1., 1.),
        Quat::from_axis_angle(Vec3::new(-0.0394818427, 0.00003351599, 1.), 3.124470974),
        Vec3::new(0., 0.355, 0.),
    ));

    InventoryItemBundle {
        inventory_item: InventoryItem {
            in_inventory_of_entity: holder_option,
            attachment_transforms: attachment_transforms,
            drop_transform: default_transform,
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
