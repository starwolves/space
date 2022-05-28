use std::{collections::HashMap, sync::Arc};

use bevy_ecs::entity::Entity;
use bevy_math::{Mat4, Quat, Vec3};
use bevy_transform::prelude::Transform;

use crate::{
    core::{
        health::components::{DamageFlag, DamageModel},
        inventory::components::SlotType,
        inventory_item::{components::InventoryItem, spawn::InventoryItemBundle},
        tab_actions::components::TabAction,
    },
    entities::construction_tool_admin::functions::{
        construct_action, construction_option_action, deconstruct_action,
    },
};

pub fn inventory_item_bundle(holder_option: Option<Entity>) -> InventoryItemBundle {
    let mut attachment_transforms = HashMap::new();
    attachment_transforms.insert(
        "left_hand".to_string(),
        Transform::from_matrix(Mat4::from_scale_rotation_translation(
            Vec3::new(0.5, 0.5, 0.5),
            Quat::from_axis_angle(Vec3::new(0.0697873, -0.966557, -0.246774), 1.8711933),
            Vec3::new(-0.047, 0.024, -0.035),
        )),
    );
    attachment_transforms.insert(
        "right_hand".to_string(),
        Transform::from_matrix(Mat4::from_scale_rotation_translation(
            Vec3::new(0.5, 0.5, 0.5),
            Quat::from_axis_angle(Vec3::new(-0.1942536, 0.9779768, 0.076334), 2.1748603),
            Vec3::new(0.042, -0., -0.021),
        )),
    );
    attachment_transforms.insert(
        "holster".to_string(),
        Transform::from_matrix(Mat4::from_scale_rotation_translation(
            Vec3::new(0.5, 0.5, 0.5),
            Quat::from_axis_angle(Vec3::new(-0.6264298, -0.1219246, 0.7698832), 2.4247889),
            Vec3::new(0., -0.093, 0.036),
        )),
    );

    let mut melee_damage_flags = HashMap::new();
    melee_damage_flags.insert(0, DamageFlag::SoftDamage);

    let default_transform = Transform::identity();

    InventoryItemBundle {
        inventory_item: InventoryItem {
            in_inventory_of_entity: holder_option,
            drop_transform: default_transform,
            active_slot_tab_actions: vec![
                TabAction {
                    id: "action::construction_tool_admin/construct".to_string(),
                    text: "Construct".to_string(),
                    tab_list_priority: 50,
                    prerequisite_check: Arc::new(construct_action),
                    belonging_entity: holder_option,
                },
                TabAction {
                    id: "action::construction_tool_admin/deconstruct".to_string(),
                    text: "Deconstruct".to_string(),
                    tab_list_priority: 49,
                    prerequisite_check: Arc::new(deconstruct_action),
                    belonging_entity: holder_option,
                },
                TabAction {
                    id: "action::construction_tool_admin/constructionoptions".to_string(),
                    text: "Construction Options".to_string(),
                    tab_list_priority: 48,
                    prerequisite_check: Arc::new(construction_option_action),
                    belonging_entity: holder_option,
                },
            ],
            attachment_transforms: attachment_transforms.clone(),
            slot_type: SlotType::Holster,
            combat_melee_damage_model: DamageModel {
                brute: 9.,
                damage_flags: melee_damage_flags.clone(),
                ..Default::default()
            },
            ..Default::default()
        },
    }
}
