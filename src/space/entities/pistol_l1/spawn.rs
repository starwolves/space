use std::collections::{BTreeMap, HashMap};

use bevy::{math::{Mat4, Quat, Vec3}, prelude::{Color, Commands, Entity, EventWriter, Transform, warn}};
use bevy_rapier3d::prelude::{CoefficientCombineRule, ColliderBundle, ColliderFlags, ColliderMaterial, ColliderPosition, ColliderShape, InteractionGroups, RigidBodyActivation, RigidBodyBundle, RigidBodyForces, RigidBodyType};

use crate::space::{entities::helmet_security::spawn::STANDARD_BODY_FRICTION, core::{inventory::components::SlotType, inventory_item::components::{InventoryItem, CombatAttackAnimation, ProjectileType, CombatType, CombatSoundSet, CombatStandardAnimation}, health::components::{DamageFlag, DamageModel, Health}, rigid_body::components::{CachedBroadcastTransform, DefaultTransform, RigidBodyDisabled, RigidBodyLinkTransform}, physics::{components::{WorldMode, WorldModes}, functions::{get_bit_masks, ColliderGroup}}, entity::{components::{EntityData, EntityUpdates, Examinable, RichName, Showcase, Sensable}, resources::{SpawnPawnData, SpawnHeldData}, functions::transform_to_isometry::transform_to_isometry, events::NetShowcase}, networking::resources::ReliableServerMessage}};

use super::components::PistolL1;

pub const PISTOL_L1_PROJECTILE_RANGE : f32 = 50.;

pub struct PistolL1Bundle;

impl PistolL1Bundle {

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


    let template_examine_text = "A standard issue laser pistol. It is a lethal weapon.".to_string();
    let mut examine_map = BTreeMap::new();
    examine_map.insert(0, template_examine_text);
    
    let mut attachment_transforms = HashMap::new();

    attachment_transforms.insert("left_hand".to_string(), Transform::from_matrix(
        Mat4::from_scale_rotation_translation(
        Vec3::new(0.5,0.5,0.5),
      Quat::from_axis_angle(Vec3::new(-0.5695359, -0.7159382, 0.4038085 ), 2.4144572),
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

    attachment_transforms.insert("holster".to_string(), Transform::from_matrix(
        Mat4::from_scale_rotation_translation(
        Vec3::new(0.5,0.5,0.5),
      Quat::from_axis_angle(Vec3::new(0.004467, 0.0995011, -0.9950274 ), 3.0523109),
   Vec3::new(0.,0.132, 0.05)
        )
    ));


    let mut builder = commands.spawn_bundle(rigid_body_component);

    let entity_id = builder.id();

    let mut melee_damage_flags = HashMap::new();
    melee_damage_flags.insert(0, DamageFlag::SoftDamage);

    let mut projectile_damage_flags = HashMap::new();
    projectile_damage_flags.insert(0, DamageFlag::WeakLethalLaser);
    
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
            name : RichName {
                name: "laser pistol".to_string(),
                n: false,
                ..Default::default()
            },
            ..Default::default()
        },
        PistolL1,
        InventoryItem {
            in_inventory_of_entity: holder_entity_option,
            attachment_transforms: attachment_transforms,
            drop_transform: default_transform,
            slot_type: SlotType::Holster,
            is_attached_when_worn : true,
            combat_attack_animation : CombatAttackAnimation::PistolShot,
            combat_type: CombatType::Projectile(ProjectileType::Laser(Color::RED, 3., 0.025, PISTOL_L1_PROJECTILE_RANGE)),
            combat_melee_damage_model : DamageModel {
                brute: 9.,
                damage_flags: melee_damage_flags,
                ..Default::default()
            },
            combat_projectile_damage_model : Some(DamageModel {
                burn: 15.,
                damage_flags: projectile_damage_flags,
                ..Default::default()
            }),
            combat_melee_sound_set: CombatSoundSet::default(),
            combat_standard_animation : CombatStandardAnimation::PistolStance,
            combat_projectile_sound_set: Some(CombatSoundSet::default_laser_projectiles()),
            combat_melee_text_set : InventoryItem::get_default_strike_words(),
            combat_projectile_text_set : Some(InventoryItem::get_default_laser_words()),
            trigger_melee_text_set: InventoryItem::get_default_trigger_melee_words(),
            trigger_projectile_text_set: Some(InventoryItem::get_default_trigger_weapon_words()),
            active_slot_tab_actions: vec![],
        },
        DefaultTransform {
            transform: default_transform,
        },
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
