use bevy::math::{Mat4, Quat, Vec3};
use bevy::prelude::{Commands, EventReader, Transform};
use bevy_xpbd_3d::prelude::{CoefficientCombine, Collider, Friction};
use combat::attack::DEFAULT_INVENTORY_ITEM_DAMAGE;
use entity::entity_macros::Identity;
use entity::entity_types::EntityType;
use entity::examine::{Examinable, RichName};
use entity::health::DamageFlag;
use entity::spawn::{BaseEntityBuilder, BaseEntityBundle, EntityBuildData, NoData, SpawnEntity};
use inventory::item::InventoryItem;
use inventory::server::combat::{DamageModel, MeleeCombat};
use inventory::server::inventory::SlotType;
use inventory::spawn_item::{InventoryItemBuilder, InventoryItemBundle};
use physics::rigid_body::STANDARD_BODY_FRICTION;
use physics::spawn::{RigidBodyBuilder, RigidBodyBundle};
use resources::core::SF_CONTENT_PREFIX;
use resources::math::Vec2Int;
use std::collections::BTreeMap;

use super::construction_tool::ConstructionTool;

pub fn get_default_transform() -> Transform {
    Transform::IDENTITY
}

impl BaseEntityBuilder<NoData> for ConstructionToolType {
    fn get_bundle(&self, _spawn_data: &EntityBuildData, _entity_data: NoData) -> BaseEntityBundle {
        let mut examine_map = BTreeMap::new();
        examine_map.insert(
            0,
            "A construction tool. Use this to construct or deconstruct ship hull cells."
                .to_string(),
        );
        BaseEntityBundle {
            default_transform: get_default_transform(),
            examinable: Examinable {
                assigned_texts: examine_map,
                name: RichName {
                    name: "construction tool".to_string(),
                    n: true,
                    ..Default::default()
                },
                ..Default::default()
            },
            entity_type: Box::new(ConstructionToolType::new()),
            ..Default::default()
        }
    }
}
use std::collections::HashMap;

impl InventoryItemBuilder for ConstructionToolType {
    fn get_bundle(&self, spawn_data: &EntityBuildData) -> InventoryItemBundle {
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

        InventoryItemBundle {
            inventory_item: InventoryItem {
                in_inventory_of_entity: spawn_data.holder_entity_option,
                drop_transform: get_default_transform(),
                attachment_transforms: attachment_transforms.clone(),
                slot_type: SlotType::Holster,
                slot_size: Vec2Int { x: 2, y: 3 },
                ..Default::default()
            },
            melee_combat: MeleeCombat {
                combat_melee_damage_model: DamageModel {
                    brute: DEFAULT_INVENTORY_ITEM_DAMAGE,
                    damage_flags: melee_damage_flags.clone(),
                    ..Default::default()
                },
                ..Default::default()
            },
            projectile_combat_option: None,
        }
    }
}

impl RigidBodyBuilder<NoData> for ConstructionToolType {
    fn get_bundle(&self, _spawn_data: &EntityBuildData, _entity_data: NoData) -> RigidBodyBundle {
        let mut friction = Friction::new(STANDARD_BODY_FRICTION);
        friction.combine_rule = CoefficientCombine::Multiply;

        RigidBodyBundle {
            collider: Collider::cuboid(0.11 * 1.5, 0.1 * 1.5, 0.13 * 1.5),
            collider_transform: Transform::from_translation(Vec3::new(0., 0.087, 0.)),
            collider_friction: friction,
            ..Default::default()
        }
    }
}

#[derive(Clone, Identity)]
pub struct ConstructionToolType {
    pub identifier: String,
}

impl Default for ConstructionToolType {
    fn default() -> Self {
        ConstructionToolType {
            identifier: SF_CONTENT_PREFIX.to_string() + "construction_tool",
        }
    }
}

pub fn build_construction_tools<T: Send + Sync + 'static>(
    mut commands: Commands,
    mut spawn_events: EventReader<SpawnEntity<T>>,
) {
    for spawn_event in spawn_events.read() {
        commands
            .entity(spawn_event.spawn_data.entity.unwrap())
            .insert(ConstructionTool::default());
    }
}
