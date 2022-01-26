use std::{collections::{BTreeMap, HashMap}, sync::Arc};

use bevy::{math::{Mat4, Quat, Vec3}, prelude::{Commands, Entity, EventWriter, Transform, warn}};
use bevy_rapier3d::prelude::{CoefficientCombineRule, ColliderBundle, ColliderFlags, ColliderMaterial, ColliderPosition, ColliderShape, InteractionGroups, RigidBodyActivation, RigidBodyBundle, RigidBodyCcd, RigidBodyForces, RigidBodyType};

use crate::space_core::{components::{cached_broadcast_transform::CachedBroadcastTransform, construction_tool::ConstructionTool, default_transform::DefaultTransform, entity_data::{EntityData}, entity_updates::EntityUpdates, examinable::{Examinable, RichName}, health::{DamageFlag, DamageModel, Health}, interpolation_priority::{InterpolationPriority}, inventory::{SlotType, Inventory}, inventory_item::{CombatAttackAnimation, CombatSoundSet, CombatStandardAnimation, CombatType, InventoryItem}, rigidbody_disabled::RigidBodyDisabled, rigidbody_link_transform::RigidBodyLinkTransform, sensable::Sensable, showcase::Showcase, world_mode::{WorldMode, WorldModes}, pawn::TabAction}, events::net::net_showcase::NetShowcase, functions::{converters::transform_to_isometry::transform_to_isometry, entity::{collider_interaction_groups::{ColliderGroup, get_bit_masks}, can_reach_entity::REACH_DISTANCE}}, resources::{entity_data_resource::{SpawnHeldData, SpawnPawnData}, network_messages::{ReliableServerMessage, GridMapType}}};

use super::helmet_security::STANDARD_BODY_FRICTION;

pub struct ConstructionToolBundle;

impl ConstructionToolBundle {

    pub fn spawn(
        passed_transform : Transform,
        commands : &mut Commands,
        correct_transform: bool,
        _pawn_data_option : Option<SpawnPawnData>,
        held_data_option : Option<SpawnHeldData>
    ) -> Entity {

        match held_data_option {
            Some(held_data) => {
                let (
                    holder_entity, 
                    showcase_instance, 
                    showcase_handle_option, 
                    net_showcase
                ) = held_data.data;
                spawn_entity(
                    commands,
        
                    None,
                
                    true,
                    Some(holder_entity),
                    showcase_instance,
                    showcase_handle_option,
                    net_showcase,
                    false,
                )
            },
            None => {
                spawn_entity(
                    commands,
        
                    Some(passed_transform),
                
                    false,
                    None,
                    false,
                    None,
                    &mut None,
                    correct_transform,
                )
            },
        }

    }
}

fn spawn_entity(

    commands : &mut Commands,

    passed_transform_option : Option<Transform>,

    held : bool,
    holder_entity_option : Option<Entity>,

    showcase_instance : bool,
    showcase_handle_option : Option<u32>,

    net_showcase : &mut Option<&mut EventWriter<NetShowcase>>,

    correct_transform : bool,
) -> Entity {

    let mut this_transform;
    let default_transform = Transform::identity();

    match passed_transform_option {
        Some(transform) => {
            this_transform = transform;
        },
        None => {
            this_transform = default_transform;
        },
    }

    if correct_transform {
        this_transform.rotation = default_transform.rotation;
    }

    let rigid_body_component;
    let collider_component;

    let shape  = ColliderShape::cuboid(
        0.11*1.5,
        0.1*1.5,
        0.13*1.5,
    );

    let collider_position : ColliderPosition = Vec3::new(0., 0.087, 0.).into();

    if held == false {

        rigid_body_component = RigidBodyBundle {
            body_type: RigidBodyType::Dynamic.into(),
            position: transform_to_isometry(this_transform).into(),
            ..Default::default()
        };

        let masks = get_bit_masks(ColliderGroup::Standard);

        collider_component = ColliderBundle {
            
            shape: shape.into(),
            position: collider_position.into(),
            material: ColliderMaterial {
                friction: STANDARD_BODY_FRICTION,
                friction_combine_rule:  CoefficientCombineRule::Multiply,
                ..Default::default()
            }.into(),
            flags: ColliderFlags {
                collision_groups: InteractionGroups::new(masks.0,masks.1),
                ..Default::default()
            }.into(),
            ..Default::default()
        };

    } else {

        rigid_body_component = RigidBodyBundle {
            body_type: RigidBodyType::Dynamic.into(),
            position: transform_to_isometry(this_transform).into(),
            forces: RigidBodyForces {
                gravity_scale: 0.,
                ..Default::default()
            }.into(),
            activation: RigidBodyActivation {
                sleeping: true,
                ..Default::default()
            }.into(),
            ..Default::default()
        };

        let masks = get_bit_masks(ColliderGroup::NoCollision);

        collider_component = ColliderBundle {
            
            shape: shape.into(),
            position: collider_position.into(),
            material: ColliderMaterial {
                friction: STANDARD_BODY_FRICTION,
                friction_combine_rule:  CoefficientCombineRule::Average,
                ..Default::default()
            }.into(),
            flags: ColliderFlags {
                collision_groups: InteractionGroups::new(masks.0,masks.1),
                ..Default::default()
            }.into(),
            ..Default::default()
        };

    }


    let template_examine_text = "A construction tool. Use this to construct or deconstruct ship hull cells.".to_string();
    let mut examine_map = BTreeMap::new();
    examine_map.insert(0, template_examine_text);
    
    let mut attachment_transforms = HashMap::new();

    attachment_transforms.insert("left_hand".to_string(), Transform::from_matrix(
        Mat4::from_scale_rotation_translation(
        Vec3::new(0.5,0.5,0.5),
      Quat::from_axis_angle(Vec3::new(0.0697873, -0.966557, -0.246774 ), 1.8711933),
      Vec3::new(-0.047,0.024, -0.035)
        )
    ));

    attachment_transforms.insert("right_hand".to_string(), Transform::from_matrix(
        Mat4::from_scale_rotation_translation(
        Vec3::new(0.5,0.5,0.5),
        Quat::from_axis_angle(Vec3::new(-0.1942536, 0.9779768, 0.076334 ), 2.1748603),
   Vec3::new(0.042,-0., -0.021)
        )
    ));

    attachment_transforms.insert("holster".to_string(), Transform::from_matrix(
        Mat4::from_scale_rotation_translation(
        Vec3::new(0.5,0.5,0.5),
      Quat::from_axis_angle(Vec3::new(-0.6264298, -0.1219246, 0.7698832 ), 2.4247889),
   Vec3::new(0.,-0.093, 0.036)
        )
    ));


    let mut builder = commands.spawn_bundle(rigid_body_component);

    let entity_id = builder.id();

    let mut melee_damage_flags = HashMap::new();
    melee_damage_flags.insert(0, DamageFlag::SoftDamage);

    let mut projectile_damage_flags = HashMap::new();
    projectile_damage_flags.insert(0, DamageFlag::WeakLethalLaser);
    
    let entity_type = "constructionTool";

    builder.insert_bundle(
        collider_component,
    ).insert_bundle((
        EntityData {
            entity_class : "entity".to_string(),
            entity_type : entity_type.to_string(),
            ..Default::default()
        },
        EntityUpdates::default(),
        WorldMode {
            mode : WorldModes::Physics
        },
        CachedBroadcastTransform::default(),
        Examinable {
            assigned_texts: examine_map,
            name : RichName {
                name: "construction tool".to_string(),
                n: false,
                ..Default::default()
            },
            ..Default::default()
        },
        ConstructionTool,
        InventoryItem {
            in_inventory_of_entity: holder_entity_option,
            attachment_transforms: attachment_transforms,
            drop_transform: default_transform,
            slot_type: SlotType::Holster,
            is_attached_when_worn : true,
            combat_attack_animation : CombatAttackAnimation::OneHandedMeleePunch,
            combat_type: CombatType::MeleeDirect,
            combat_melee_damage_model : DamageModel {
                brute: 9.,
                damage_flags: melee_damage_flags,
                ..Default::default()
            },
            combat_projectile_damage_model : None,
            combat_melee_sound_set: CombatSoundSet::default(),
            combat_standard_animation : CombatStandardAnimation::StandardStance,
            combat_projectile_sound_set: None,
            combat_melee_text_set : InventoryItem::get_default_strike_words(),
            combat_projectile_text_set : None,
            trigger_melee_text_set: InventoryItem::get_default_trigger_melee_words(),
            trigger_projectile_text_set: None,
            active_slot_tab_actions: vec![
                TabAction {
                    id: "construct".to_string(),
                    text: "Construct".to_string(),
                    tab_list_priority: 50,
                    prerequisite_check: Arc::new(construct_action),
                },
                TabAction {
                    id: "deconstruct".to_string(),
                    text: "Deconstruct".to_string(),
                    tab_list_priority: 49,
                    prerequisite_check: Arc::new(deconstruct_action),
                },
            ]
        },
        DefaultTransform {
            transform: default_transform,
        },
        InterpolationPriority::default(),
    ));

    if showcase_instance {
        let handle = showcase_handle_option.unwrap();
        builder.insert(
            Showcase {
                handle: handle,
            }
        );
        let entity_updates = HashMap::new();
        net_showcase.as_deref_mut().unwrap().send(NetShowcase{
            handle: handle,
            message: ReliableServerMessage::LoadEntity(
                "entity".to_string(),
                entity_type.to_string(),
                entity_updates,
                entity_id.to_bits(),
                true,
                "main".to_string(),
                "".to_string(),
                false,
            )
        });
    } else {
        builder.insert_bundle((
            Sensable::default(),
            Health::default(),
        ));
    }

    match held {
        true => {
            builder.insert_bundle((
                RigidBodyDisabled,
                WorldMode {
                    mode : WorldModes::Worn
                },
            ));
        },
        false => {
            builder.insert(
                WorldMode {
                    mode : WorldModes::Physics
                },
            );
        },
    }

    match holder_entity_option {
        Some(holder_entity) => {
            builder.insert(RigidBodyLinkTransform{
                follow_entity: holder_entity,
                ..Default::default()
            });
        },
        None => {
            if held == true {
                warn!("Spawned entity in held mode but holder_entity_option is none.");
            }
        },
    }

    entity_id

}

pub fn construct_action(
    _entity_id_bits_option : Option<u64>,
    cell_id_option : Option<(GridMapType, i16,i16,i16)>,
    distance : f32,
    _inventory_component : &Inventory,
) -> bool {
    distance < REACH_DISTANCE && cell_id_option.is_some()
}

pub fn deconstruct_action(
    _entity_id_bits_option : Option<u64>,
    cell_id_option : Option<(GridMapType, i16,i16,i16)>,
    distance : f32,
    _inventory_component : &Inventory,
) -> bool {
    distance < REACH_DISTANCE && cell_id_option.is_some()
}
