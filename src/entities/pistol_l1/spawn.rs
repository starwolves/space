use std::collections::{BTreeMap, HashMap};

use bevy_ecs::entity::Entity;
use bevy_hierarchy::BuildChildren;
use bevy_log::warn;
use bevy_math::{Mat4, Quat, Vec3};
use bevy_rapier3d::prelude::{
    CoefficientCombineRule, Collider, CollisionGroups, ExternalForce, ExternalImpulse, Friction,
    GravityScale, RigidBody, Sleeping, Velocity,
};
use bevy_transform::components::Transform;

use crate::core::{
    entity::{
        components::{EntityData, EntityUpdates, Showcase},
        events::NetShowcase,
        resources::SpawnData,
    },
    examinable::components::{Examinable, RichName},
    health::components::{DamageFlag, DamageModel, Health},
    inventory::components::SlotType,
    inventory_item::components::{
        CombatAttackAnimation, CombatSoundSet, CombatStandardAnimation, CombatType, InventoryItem,
        ProjectileType,
    },
    networking::resources::ReliableServerMessage,
    physics::{
        components::{WorldMode, WorldModes},
        functions::{get_bit_masks, ColliderGroup},
    },
    rigid_body::components::{
        CachedBroadcastTransform, RigidBodyData, RigidBodyDisabled, RigidBodyLinkTransform,
        STANDARD_BODY_FRICTION,
    },
    sensable::components::Sensable,
};

use super::components::PistolL1;

pub const PISTOL_L1_PROJECTILE_RANGE: f32 = 50.;

pub struct PistolL1Bundle;

impl PistolL1Bundle {
    pub fn spawn(spawn_data: SpawnData) -> Entity {
        let mut this_transform;
        let default_transform = Transform::from_matrix(Mat4::from_scale_rotation_translation(
            Vec3::new(1., 1., 1.),
            Quat::from_axis_angle(Vec3::new(0.07410704, 0.07611039, -0.99434173), 4.7049665),
            Vec3::new(0., 0.355, 0.),
        ));

        this_transform = spawn_data.entity_transform;

        if spawn_data.correct_transform {
            this_transform.rotation = default_transform.rotation;
        }
        let friction_val = STANDARD_BODY_FRICTION;
        let friction_combine_rule = CoefficientCombineRule::Multiply;

        let shape = Collider::cuboid(0.047, 0.219, 0.199);

        let collider_position = Vec3::new(0., 0.087, 0.).into();

        let rigid_body = RigidBody::Dynamic;

        let mut builder = spawn_data.commands.spawn();

        builder
            .insert(rigid_body)
            .insert(this_transform)
            .insert(Velocity::default())
            .insert(ExternalForce::default())
            .insert(ExternalImpulse::default());

        let mut friction = Friction::coefficient(friction_val);
        friction.combine_rule = friction_combine_rule;

        let t = Transform::from_translation(collider_position);

        let held = spawn_data.held_data_option.is_some();

        if held == false {
            let masks = get_bit_masks(ColliderGroup::Standard);

            builder
                .insert(Sleeping::default())
                .insert(GravityScale::default())
                .with_children(|children| {
                    children
                        .spawn()
                        .insert(shape)
                        .insert(t)
                        .insert(friction)
                        .insert(CollisionGroups::new(masks.0, masks.1));
                });
        } else {
            let masks = get_bit_masks(ColliderGroup::NoCollision);

            let sleeping = Sleeping {
                sleeping: true,
                ..Default::default()
            };

            builder
                .insert(GravityScale(0.))
                .insert(sleeping)
                .with_children(|children| {
                    children
                        .spawn()
                        .insert(shape)
                        .insert(t)
                        .insert(friction)
                        .insert(CollisionGroups::new(masks.0, masks.1));
                });
        }

        let template_examine_text =
            "A standard issue laser pistol. It is a lethal weapon.".to_string();
        let mut examine_map = BTreeMap::new();
        examine_map.insert(0, template_examine_text);

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

        let entity_id = builder.id();

        let mut melee_damage_flags = HashMap::new();
        melee_damage_flags.insert(0, DamageFlag::SoftDamage);

        let mut projectile_damage_flags = HashMap::new();
        projectile_damage_flags.insert(0, DamageFlag::WeakLethalLaser);

        builder.insert_bundle((
        EntityData {
            entity_class: "entity".to_string(),
            entity_name: "pistolL1".to_string(),
            ..Default::default()
        },
        EntityUpdates::default(),
        WorldMode {
            mode: WorldModes::Physics,
        },
        CachedBroadcastTransform::default(),
        Examinable {
            assigned_texts: examine_map,
            name: RichName {
                name: "laser pistol".to_string(),
                n: false,
                ..Default::default()
            },
            ..Default::default()
        },
        PistolL1,
        InventoryItem {
            in_inventory_of_entity: spawn_data.held_data_option,
            attachment_transforms: attachment_transforms,
            drop_transform: default_transform,
            slot_type: SlotType::Holster,
            is_attached_when_worn: true,
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
            combat_melee_sound_set: CombatSoundSet::default(),
            combat_standard_animation: CombatStandardAnimation::PistolStance,
            combat_projectile_sound_set: Some(CombatSoundSet::default_laser_projectiles()),
            combat_melee_text_set: InventoryItem::get_default_strike_words(),
            combat_projectile_text_set: Some(InventoryItem::get_default_laser_words()),
            trigger_melee_text_set: InventoryItem::get_default_trigger_melee_words(),
            trigger_projectile_text_set: Some(InventoryItem::get_default_trigger_weapon_words()),
            active_slot_tab_actions: vec![],
            throw_force_factor: 1.,
        },
        RigidBodyData {
            friction: friction.coefficient,
            friction_combine_rule: friction.combine_rule,
        },
    ));

        if spawn_data.showcase_data_option.is_some() {
            let handle = spawn_data.showcase_data_option.as_mut().unwrap();
            builder.insert(Showcase {
                handle: handle.handle,
            });
            let entity_updates = HashMap::new();
            handle.event_writer.send(NetShowcase {
                handle: handle.handle,
                message: ReliableServerMessage::LoadEntity(
                    "entity".to_string(),
                    "pistolL1".to_string(),
                    entity_updates,
                    entity_id.to_bits(),
                    true,
                    "main".to_string(),
                    "".to_string(),
                    false,
                ),
            });
        } else {
            builder.insert_bundle((Sensable::default(), Health::default()));
        }

        match held {
            true => {
                builder.insert_bundle((
                    RigidBodyDisabled,
                    WorldMode {
                        mode: WorldModes::Worn,
                    },
                ));
            }
            false => {
                builder.insert(WorldMode {
                    mode: WorldModes::Physics,
                });
            }
        }

        match spawn_data.held_data_option {
            Some(holder_entity) => {
                builder.insert(RigidBodyLinkTransform {
                    follow_entity: holder_entity,
                    ..Default::default()
                });
            }
            None => {
                if held == true {
                    warn!("Spawned entity in held mode but holder_entity_option is none.");
                }
            }
        }

        entity_id
    }
}
