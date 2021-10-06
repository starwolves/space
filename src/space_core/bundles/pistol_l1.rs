use std::collections::{BTreeMap, HashMap};

use bevy::{math::{Mat4, Quat, Vec3}, prelude::{Commands, Entity, EventWriter, Transform, warn}};
use bevy_rapier3d::prelude::{CoefficientCombineRule, ColliderBundle, ColliderFlags, ColliderMaterial, ColliderPosition, ColliderShape, InteractionGroups, RigidBodyActivation, RigidBodyBundle, RigidBodyCcd, RigidBodyForces, RigidBodyType};

use crate::space_core::{components::{cached_broadcast_transform::CachedBroadcastTransform, default_transform::DefaultTransform, entity_data::{EntityData}, entity_updates::EntityUpdates, examinable::Examinable, health::{DamageFlag, DamageModel, Health}, interpolation_priority::{InterpolationPriority}, inventory::SlotType, inventory_item::{CombatAnimation, CombatType, InventoryItem, MeleeCombatSoundSet}, pistol_l1::PistolL1, rigidbody_disabled::RigidBodyDisabled, rigidbody_link_transform::RigidBodyLinkTransform, sensable::Sensable, showcase::Showcase, world_mode::{WorldMode, WorldModes}}, events::net::net_showcase::NetShowcase, functions::{converters::transform_to_isometry::transform_to_isometry, entity::{collider_interaction_groups::{ColliderGroup, get_bit_masks}}}, resources::network_messages::ReliableServerMessage};

use super::helmet_security::STANDARD_BODY_FRICTION;


pub struct PistolL1Bundle;

impl PistolL1Bundle {

    pub fn spawn_held(
        commands : &mut Commands,
        holder_entity : Entity,
        showcase_instance : bool,
        showcase_handle_option : Option<u32>,
        net_showcase : &mut Option<&mut EventWriter<NetShowcase>>,
    ) -> Entity {

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

    }

    pub fn spawn(
        passed_transform : Transform,
        commands : &mut Commands,
        correct_transform: bool,
    ) -> Entity {

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
    let default_transform = Transform::from_matrix(
    Mat4::from_scale_rotation_translation(
Vec3::new(1.,1.,1.),
    Quat::from_axis_angle(Vec3::new(0.07410704, 0.07611039, -0.99434173), 4.7049665),
Vec3::new(0.,0.355, 0.)
    ),);

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
        0.047,
        0.219,
        0.199,
    );

    let collider_position : ColliderPosition = Vec3::new(0., 0.087, 0.).into();

    if held == false {

        rigid_body_component = RigidBodyBundle {
            body_type: RigidBodyType::Dynamic,
            position: transform_to_isometry(this_transform).into(),
            ccd: RigidBodyCcd {
                ccd_enabled: false,
                ..Default::default()
            },
            ..Default::default()
        };

        let masks = get_bit_masks(ColliderGroup::Standard);

        collider_component = ColliderBundle {
            
            shape: shape,
            position: collider_position,
            material: ColliderMaterial {
                friction: STANDARD_BODY_FRICTION,
                friction_combine_rule:  CoefficientCombineRule::Average,
                ..Default::default()
            },
            flags: ColliderFlags {
                collision_groups: InteractionGroups::new(masks.0,masks.1),
                ..Default::default()
            },
            ..Default::default()
        };

    } else {

        rigid_body_component = RigidBodyBundle {
            body_type: RigidBodyType::Dynamic,
            position: transform_to_isometry(this_transform).into(),
            ccd: RigidBodyCcd {
                ccd_enabled: false,
                ..Default::default()
            },
            forces: RigidBodyForces {
                gravity_scale: 0.,
                ..Default::default()
            },
            activation: RigidBodyActivation {
                sleeping: true,
                ..Default::default()
            },
            ..Default::default()
        };

        let masks = get_bit_masks(ColliderGroup::NoCollision);

        collider_component = ColliderBundle {
            
            shape: shape,
            position: collider_position,
            material: ColliderMaterial {
                friction: STANDARD_BODY_FRICTION,
                friction_combine_rule:  CoefficientCombineRule::Average,
                ..Default::default()
            },
            flags: ColliderFlags {
                collision_groups: InteractionGroups::new(masks.0,masks.1),
                ..Default::default()
            },
            ..Default::default()
        };

    }


    let template_examine_text = "A standard issue laser pistol. It is a lethal weapon.".to_string();
    let mut examine_map = BTreeMap::new();
    examine_map.insert(0, template_examine_text);
    
    let mut attachment_transforms = HashMap::new();

    attachment_transforms.insert("left_hand".to_string(), Transform::from_matrix(
        Mat4::from_scale_rotation_translation(
        Vec3::new(0.5,0.5,0.5),
      Quat::from_axis_angle(Vec3::new(0.4647392, 0.8599459, -0.210975 ), 2.5530127),
   Vec3::new(-0.031,0.033, 0.011)
        )
    ));

    attachment_transforms.insert("right_hand".to_string(), Transform::from_matrix(
        Mat4::from_scale_rotation_translation(
        Vec3::new(0.5,0.5,0.5),
        Quat::from_xyzw(0.611671 , 0.396847 , 0.530651 , 0.432181),
   Vec3::new(0.077,-0.067, -0.045)
        )
    ));


    let mut builder = commands.spawn_bundle(rigid_body_component);

    let entity_id = builder.id();

    let mut first_damage_flags = HashMap::new();
    first_damage_flags.insert(0, DamageFlag::SoftDamage);
    
    builder.insert_bundle(
        collider_component,
    ).insert_bundle((
        EntityData {
            entity_class : "entity".to_string(),
            entity_type : "pistolL1".to_string(),
            ..Default::default()
        },
        EntityUpdates::default(),
        WorldMode {
            mode : WorldModes::Physics
        },
        CachedBroadcastTransform::default(),
        Examinable {
            assigned_texts: examine_map,
            name: "a laser pistol".to_string(),
            ..Default::default()
        },
        PistolL1,
        InventoryItem {
            in_inventory_of_entity: holder_entity_option,
            attachment_transforms: attachment_transforms,
            drop_transform: default_transform,
            slot_type: SlotType::Generic,
            is_attached_when_worn : true,
            combat_animation : CombatAnimation::OneHandedMeleePunch,
            combat_type: CombatType::MeleeDirect,
            combat_damage_model : DamageModel {
                brute: 9.,
                damage_flags: first_damage_flags,
                ..Default::default()
            },
            combat_sound_set: MeleeCombatSoundSet::default(),
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
                "pistolL1".to_string(),
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
