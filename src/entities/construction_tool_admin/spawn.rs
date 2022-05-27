use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
};

use bevy_ecs::{entity::Entity, system::Query};
use bevy_hierarchy::BuildChildren;
use bevy_log::warn;
use bevy_math::{Mat4, Quat, Vec3};
use bevy_rapier3d::prelude::{
    CoefficientCombineRule, Collider, CollisionGroups, ExternalForce, ExternalImpulse, Friction,
    GravityScale, RigidBody, Sleeping, Velocity,
};
use bevy_transform::components::Transform;

use crate::{
    core::{
        data_link::components::DataLink,
        entity::{
            components::{EntityData, EntityUpdates, Showcase},
            events::NetShowcase,
            resources::{EntityDataResource, SpawnData},
        },
        examinable::components::{Examinable, RichName},
        gridmap::resources::CellData,
        health::components::{DamageFlag, DamageModel, Health},
        inventory::components::{Inventory, SlotType},
        inventory_item::components::{
            CombatAttackAnimation, CombatSoundSet, CombatStandardAnimation, CombatType,
            InventoryItem,
        },
        networking::resources::{GridMapType, ReliableServerMessage},
        pawn::functions::can_reach_entity::REACH_DISTANCE,
        physics::{
            components::{WorldMode, WorldModes},
            functions::{get_bit_masks, ColliderGroup},
        },
        rigid_body::components::{
            CachedBroadcastTransform, DefaultTransform, RigidBodyData, RigidBodyDisabled,
            RigidBodyLinkTransform,
        },
        sensable::components::Sensable,
        tab_actions::components::TabAction,
    },
    entities::{
        construction_tool_admin::components::ConstructionTool,
        helmet_security::spawn::STANDARD_BODY_FRICTION,
    },
};

pub struct ConstructionToolBundle;

impl ConstructionToolBundle {
    pub fn spawn(spawn_data: SpawnData) -> Entity {
        let mut this_transform;
        let default_transform = Transform::identity();

        let held = spawn_data.held_data_option.is_some();

        this_transform = spawn_data.entity_transform;

        if spawn_data.correct_transform {
            this_transform.rotation = default_transform.rotation;
        }

        let shape = Collider::cuboid(0.11 * 1.5, 0.1 * 1.5, 0.13 * 1.5);

        let collider_position = Vec3::new(0., 0.087, 0.).into();

        let friction_val = STANDARD_BODY_FRICTION;
        let friction_combine_rule = CoefficientCombineRule::Multiply;

        let mut t = Transform::from_translation(this_transform.translation);
        t.rotation = this_transform.rotation;
        let mut friction = Friction::coefficient(friction_val);
        friction.combine_rule = friction_combine_rule;

        let mut builder = spawn_data.commands.spawn();
        if held == false {
            let rigid_body = RigidBody::Fixed;

            let masks = get_bit_masks(ColliderGroup::Standard);

            builder
                .insert(rigid_body)
                .insert(t)
                .insert(Velocity::default())
                .insert(ExternalForce::default())
                .insert(Sleeping::default())
                .insert(GravityScale::default())
                .insert(ExternalImpulse::default())
                .with_children(|children| {
                    children
                        .spawn()
                        .insert(shape)
                        .insert(Transform::from_translation(collider_position))
                        .insert(friction)
                        .insert(CollisionGroups::new(masks.0, masks.1));
                });
        } else {
            let rigid_body = RigidBody::Dynamic;

            let masks = get_bit_masks(ColliderGroup::NoCollision);

            let sleeping = Sleeping {
                sleeping: true,
                ..Default::default()
            };

            builder
                .insert(ExternalImpulse::default())
                .insert(ExternalForce::default())
                .insert(Velocity::default())
                .insert(rigid_body)
                .insert(t)
                .insert(GravityScale(0.))
                .insert(sleeping)
                .with_children(|children| {
                    children
                        .spawn()
                        .insert(shape)
                        .insert(Transform::from_translation(collider_position))
                        .insert(friction)
                        .insert(CollisionGroups::new(masks.0, masks.1));
                });
        }

        let template_examine_text =
            "A construction tool. Use this to construct or deconstruct ship hull cells."
                .to_string();
        let mut examine_map = BTreeMap::new();
        examine_map.insert(0, template_examine_text);

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

        let entity_id = builder.id();

        let mut melee_damage_flags = HashMap::new();
        melee_damage_flags.insert(0, DamageFlag::SoftDamage);

        let mut projectile_damage_flags = HashMap::new();
        projectile_damage_flags.insert(0, DamageFlag::WeakLethalLaser);

        let entity_type = "constructionTool";

        let holder_entity_option;

        match spawn_data.held_data_option {
            Some(d) => {
                holder_entity_option = Some(d.entity);
            }
            None => {
                holder_entity_option = None;
            }
        }

        builder.insert_bundle((
            EntityData {
                entity_class: "entity".to_string(),
                entity_name: entity_type.to_string(),
                ..Default::default()
            },
            EntityUpdates::default(),
            CachedBroadcastTransform::default(),
            Examinable {
                assigned_texts: examine_map,
                name: RichName {
                    name: "admin construction tool".to_string(),
                    n: false,
                    ..Default::default()
                },
                ..Default::default()
            },
            ConstructionTool::default(),
            InventoryItem {
                in_inventory_of_entity: holder_entity_option,
                attachment_transforms: attachment_transforms,
                drop_transform: default_transform,
                slot_type: SlotType::Holster,
                is_attached_when_worn: true,
                combat_attack_animation: CombatAttackAnimation::OneHandedMeleePunch,
                combat_type: CombatType::MeleeDirect,
                combat_melee_damage_model: DamageModel {
                    brute: 9.,
                    damage_flags: melee_damage_flags,
                    ..Default::default()
                },
                combat_projectile_damage_model: None,
                combat_melee_sound_set: CombatSoundSet::default(),
                combat_standard_animation: CombatStandardAnimation::StandardStance,
                combat_projectile_sound_set: None,
                combat_melee_text_set: InventoryItem::get_default_strike_words(),
                combat_projectile_text_set: None,
                trigger_melee_text_set: InventoryItem::get_default_trigger_melee_words(),
                trigger_projectile_text_set: None,
                active_slot_tab_actions: vec![
                    TabAction {
                        id: "action::construction_tool_admin/construct".to_string(),
                        text: "Construct".to_string(),
                        tab_list_priority: 50,
                        prerequisite_check: Arc::new(construct_action),
                        belonging_entity: Some(entity_id),
                    },
                    TabAction {
                        id: "action::construction_tool_admin/deconstruct".to_string(),
                        text: "Deconstruct".to_string(),
                        tab_list_priority: 49,
                        prerequisite_check: Arc::new(deconstruct_action),
                        belonging_entity: Some(entity_id),
                    },
                    TabAction {
                        id: "action::construction_tool_admin/constructionoptions".to_string(),
                        text: "Construction Options".to_string(),
                        tab_list_priority: 48,
                        prerequisite_check: Arc::new(construction_option_action),
                        belonging_entity: Some(entity_id),
                    },
                ],
                throw_force_factor: 1.,
            },
            RigidBodyData {
                friction: friction.coefficient,
                friction_combine_rule: friction.combine_rule,
            },
            DefaultTransform {
                transform: default_transform,
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
                    entity_type.to_string(),
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

        match holder_entity_option {
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

pub fn construct_action(
    _self_tab_entity: Option<Entity>,
    _entity_id_bits_option: Option<u64>,
    cell_id_option: Option<(GridMapType, i16, i16, i16, Option<&CellData>)>,
    distance: f32,
    _inventory_component: &Inventory,
    _entity_data_resource: &EntityDataResource,
    _entity_datas: &Query<&EntityData>,
    _data_link_component: &DataLink,
) -> bool {
    distance < REACH_DISTANCE && cell_id_option.is_some()
}

pub fn deconstruct_action(
    _self_tab_entity: Option<Entity>,
    entity_id_bits_option: Option<u64>,
    cell_id_option: Option<(GridMapType, i16, i16, i16, Option<&CellData>)>,
    distance: f32,
    _inventory_component: &Inventory,
    entity_data_resource: &EntityDataResource,
    entity_datas: &Query<&EntityData>,
    _data_link_component: &DataLink,
) -> bool {
    match entity_id_bits_option {
        Some(bits) => {
            let entity = Entity::from_bits(bits);

            let mut deconstructable = false;

            match entity_datas.get(entity) {
                Ok(entity_data) => {
                    let entity_properties = entity_data_resource
                        .data
                        .get(
                            *entity_data_resource
                                .name_to_id
                                .get(&entity_data.entity_name)
                                .unwrap(),
                        )
                        .unwrap();

                    deconstructable = entity_properties.grid_item.is_some();
                }
                Err(_) => {}
            }

            distance < REACH_DISTANCE && deconstructable
        }
        None => {
            distance < REACH_DISTANCE
                && cell_id_option.is_some()
                && cell_id_option.unwrap().4.is_some()
        }
    }
}

pub fn construction_option_action(
    self_tab_entity_option: Option<Entity>,
    belonging_entity_id_bits_option: Option<u64>,
    _cell_id_option: Option<(GridMapType, i16, i16, i16, Option<&CellData>)>,
    _distance: f32,
    inventory_component: &Inventory,
    _entity_data_resource: &EntityDataResource,
    _entity_datas: &Query<&EntityData>,
    _data_link_component: &DataLink,
) -> bool {
    let is_self;

    match belonging_entity_id_bits_option {
        Some(e) => {
            let entity = Entity::from_bits(e);

            match self_tab_entity_option {
                Some(self_tab_entity) => {
                    if self_tab_entity != entity {
                        is_self = false;
                    } else {
                        if inventory_component.has_item(entity) {
                            is_self = true;
                        } else {
                            is_self = false;
                        }
                    }
                }
                None => {
                    is_self = false;
                }
            }
        }
        None => {
            is_self = false;
        }
    }

    is_self
}
