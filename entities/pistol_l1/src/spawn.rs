use bevy::math::Mat4;
use bevy::math::Quat;
use bevy::math::Vec3;
use bevy::prelude::Commands;
use bevy::prelude::EventReader;
use bevy::prelude::Transform;
use bevy_xpbd_3d::prelude::CoefficientCombine;
use bevy_xpbd_3d::prelude::Collider;
use bevy_xpbd_3d::prelude::Friction;
use combat::attack::DEFAULT_INVENTORY_ITEM_DAMAGE;
use entity::entity_macros::Identity;
use entity::entity_types::EntityType;
use entity::examine::Examinable;
use entity::examine::RichName;
use entity::health::DamageFlag;
use entity::spawn::BaseEntityBuilder;
use entity::spawn::BaseEntityBundle;
use entity::spawn::EntityBuildData;
use entity::spawn::NoData;
use entity::spawn::SpawnEntity;
use inventory::item::CombatStandardAnimation;
use inventory::item::InventoryItem;
use inventory::server::combat::CombatAttackAnimation;
use inventory::server::combat::DamageModel;
use inventory::server::combat::MeleeCombat;
use inventory::server::combat::ProjectileCombat;
use inventory::server::inventory::SlotType;
use inventory::spawn_item::InventoryItemBuilder;
use inventory::spawn_item::InventoryItemBundle;
use physics::rigid_body::STANDARD_BODY_FRICTION;
use physics::spawn::RigidBodyBuilder;
use physics::spawn::RigidBodyBundle;
use resources::core::SF_CONTENT_PREFIX;
use std::collections::BTreeMap;
use text_api::core::Color;

pub fn get_default_transform() -> Transform {
    Transform::from_matrix(Mat4::from_scale_rotation_translation(
        Vec3::new(1., 1., 1.),
        Quat::from_axis_angle(Vec3::new(-0.00000035355248, 0.707105, 0.7071085), 3.1415951),
        Vec3::new(0., 0.116, 0.),
    ))
}

impl BaseEntityBuilder<NoData> for PistolL1Type {
    fn get_bundle(&self, _spawn_data: &EntityBuildData, _entity_data: NoData) -> BaseEntityBundle {
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
            entity_type: Box::new(PistolL1Type::new()),

            ..Default::default()
        }
    }
}
use std::collections::HashMap;

pub const PISTOL_L1_PROJECTILE_RANGE: f32 = 50.;

impl InventoryItemBuilder for PistolL1Type {
    fn get_bundle(&self, spawn_data: &EntityBuildData) -> InventoryItemBundle {
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

impl RigidBodyBuilder<NoData> for PistolL1Type {
    fn get_bundle(&self, _spawn_data: &EntityBuildData, _entity_data: NoData) -> RigidBodyBundle {
        let mut friction = Friction::new(STANDARD_BODY_FRICTION);
        friction.combine_rule = CoefficientCombine::Multiply;

        RigidBodyBundle {
            collider: Collider::cuboid(0.047, 0.219, 0.199),
            collider_transform: Transform::from_translation(Vec3::new(0., 0.087, 0.)),
            collider_friction: friction,

            ..Default::default()
        }
    }
}

use super::pistol_l1::PistolL1;

#[derive(Clone, Identity)]
pub struct PistolL1Type {
    pub identifier: String,
}
impl Default for PistolL1Type {
    fn default() -> Self {
        PistolL1Type {
            identifier: SF_CONTENT_PREFIX.to_owned() + "pistol_l1",
        }
    }
}

pub fn build_pistols_l1<T: Send + Sync + 'static>(
    mut commands: Commands,
    mut spawn_events: EventReader<SpawnEntity<T>>,
) {
    for spawn_event in spawn_events.read() {
        commands
            .entity(spawn_event.spawn_data.entity)
            .insert(PistolL1);
    }
}
