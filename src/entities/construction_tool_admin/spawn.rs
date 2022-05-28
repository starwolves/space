use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
};

use bevy_ecs::{entity::Entity, system::Query};
use bevy_math::{Mat4, Quat, Vec3};
use bevy_rapier3d::prelude::{CoefficientCombineRule, Collider, Friction};
use bevy_transform::components::Transform;

use crate::{
    core::{
        data_link::components::DataLink,
        entity::{
            components::EntityData,
            functions::builder::{
                base_entity_builder, showcase_builder, BaseEntityData, EntityBundle,
                ShowCaseBuilderData,
            },
            resources::{EntityDataResource, SpawnData},
        },
        examinable::components::{Examinable, RichName},
        gridmap::resources::CellData,
        health::components::{DamageFlag, DamageModel},
        inventory::components::{Inventory, SlotType},
        inventory_item::{
            components::{
                CombatAttackAnimation, CombatSoundSet, CombatStandardAnimation, CombatType,
                InventoryItem,
            },
            functions::{inventory_item_builder, InventoryBuilderData, InventoryItemBundle},
        },
        networking::resources::GridMapType,
        pawn::functions::can_reach_entity::REACH_DISTANCE,
        rigid_body::functions::{rigidbody_builder, RigidBodySpawnData, RigidbodyBundle},
        tab_actions::components::TabAction,
    },
    entities::{
        construction_tool_admin::components::ConstructionTool,
        helmet_security::spawn::STANDARD_BODY_FRICTION,
    },
};

pub struct ConstructionToolBundle {
    entity_bundle: EntityBundle,
    rigidbody_bundle: RigidbodyBundle,
    inventory_item_bundle: InventoryItemBundle,
}

impl ConstructionToolBundle {
    pub fn spawn(mut spawn_data: SpawnData) -> Entity {
        let default_transform = Transform::identity();
        if spawn_data.correct_transform {
            spawn_data.entity_transform.rotation = default_transform.rotation;
        }

        let shape = Collider::cuboid(0.11 * 1.5, 0.1 * 1.5, 0.13 * 1.5);

        let collider_position = Vec3::new(0., 0.087, 0.).into();

        let friction_val = STANDARD_BODY_FRICTION;
        let friction_combine_rule = CoefficientCombineRule::Multiply;

        let mut t = Transform::from_translation(spawn_data.entity_transform.translation);
        t.rotation = spawn_data.entity_transform.rotation;
        let mut friction = Friction::coefficient(friction_val);
        friction.combine_rule = friction_combine_rule;

        let entity = spawn_data.commands.spawn().id();

        let entity_type = "constructionTool";

        let template_examine_text =
            "A construction tool. Use this to construct or deconstruct ship hull cells."
                .to_string();

        let mut examine_map = BTreeMap::new();
        examine_map.insert(0, template_examine_text);

        // Iventoryitem comps.
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

        let mut projectile_damage_flags = HashMap::new();
        projectile_damage_flags.insert(0, DamageFlag::WeakLethalLaser);

        rigidbody_builder(
            &mut spawn_data.commands,
            entity,
            RigidBodySpawnData {
                rigidbody_dynamic: true,
                rigid_transform: t,
                entity_is_stored_item: spawn_data.held_data_option.is_some(),
                collider: shape,
                collider_transform: Transform::from_translation(collider_position),
                collider_friction: friction,
                ..Default::default()
            },
        );

        base_entity_builder(
            &mut spawn_data.commands,
            entity,
            BaseEntityData {
                dynamicbody: true,
                entity_type: entity_type.to_string(),
                examinable: Examinable {
                    assigned_texts: examine_map,
                    name: RichName {
                        name: "admin construction tool".to_string(),
                        n: false,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
        );

        inventory_item_builder(
            &mut spawn_data.commands,
            entity,
            InventoryBuilderData {
                inventory_item: InventoryItem {
                    in_inventory_of_entity: spawn_data.held_data_option,
                    attachment_transforms: attachment_transforms.clone(),
                    drop_transform: default_transform,
                    slot_type: SlotType::Holster,
                    is_attached_when_worn: true,
                    combat_attack_animation: CombatAttackAnimation::OneHandedMeleePunch,
                    combat_type: CombatType::MeleeDirect,
                    combat_melee_damage_model: DamageModel {
                        brute: 9.,
                        damage_flags: melee_damage_flags.clone(),
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
                            belonging_entity: Some(entity),
                        },
                        TabAction {
                            id: "action::construction_tool_admin/deconstruct".to_string(),
                            text: "Deconstruct".to_string(),
                            tab_list_priority: 49,
                            prerequisite_check: Arc::new(deconstruct_action),
                            belonging_entity: Some(entity),
                        },
                        TabAction {
                            id: "action::construction_tool_admin/constructionoptions".to_string(),
                            text: "Construction Options".to_string(),
                            tab_list_priority: 48,
                            prerequisite_check: Arc::new(construction_option_action),
                            belonging_entity: Some(entity),
                        },
                    ],
                    throw_force_factor: 1.,
                },
                holder_entity_option: spawn_data.held_data_option,
            },
        );

        showcase_builder(
            &mut spawn_data.commands,
            entity,
            spawn_data.showcase_data_option,
            ShowCaseBuilderData {
                entity_type: entity_type.to_string(),
                entity_updates: HashMap::new(),
            },
        );

        spawn_data
            .commands
            .entity(entity)
            .insert(ConstructionTool::default());

        entity
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
