use std::collections::HashMap;

use bevy::{math::{Mat4, Quat, Vec3}, prelude::{Commands, Entity, EventWriter, Transform, warn}};
use bevy_rapier3d::prelude::{CoefficientCombineRule, ColliderBundle, ColliderFlags, ColliderMaterial, ColliderShape, InteractionGroups, RigidBodyActivation, RigidBodyBundle, RigidBodyCcd, RigidBodyForces, RigidBodyType};

use crate::space_core::{components::{cached_broadcast_transform::CachedBroadcastTransform, entity_data::{EntityData, EntityGroup}, entity_updates::EntityUpdates, examinable::Examinable, helmet::Helmet, inventory::SlotType, inventory_item::InventoryItem, rigidbody_disabled::RigidBodyDisabled, rigidbody_link_transform::RigidBodyLinkTransform, sensable::Sensable, showcase::Showcase, world_mode::{WorldMode, WorldModes}}, events::net::net_showcase::NetShowcase, functions::{collider_interaction_groups::{ColliderGroup, get_bit_masks}, new_chat_message::{FURTHER_ITALIC_FONT, FURTHER_NORMAL_FONT}, transform_to_isometry::transform_to_isometry}, structs::network_messages::ReliableServerMessage};


pub struct HelmetSecurityBundle;

impl HelmetSecurityBundle {

    pub fn spawn_held(
        commands : &mut Commands,
        holder_entity : Entity,
        showcase_instance : bool,
        showcase_handle_option : Option<u32>,
        net_showcase : &mut Option<&mut EventWriter<NetShowcase>>,
    ) -> Entity {

        spawn(
            commands,

            None,
        
            true,
            Some(holder_entity),
            showcase_instance,
            showcase_handle_option,
            net_showcase,
        )

    }

    pub fn spawn(
        passed_transform : Transform,
        commands : &mut Commands,
    ) -> Entity {

        spawn(
            commands,

            Some(passed_transform),
        
            false,
            None,
            false,
            None,
            &mut None,
        )

    }
}

fn spawn(

    commands : &mut Commands,

    passed_transform_option : Option<Transform>,

    held : bool,
    holder_entity_option : Option<Entity>,

    showcase_instance : bool,
    showcase_handle_option : Option<u32>,

    net_showcase : &mut Option<&mut EventWriter<NetShowcase>>,
) -> Entity {

    let this_transform;

    match passed_transform_option {
        Some(transform) => {
            this_transform = transform;
        },
        None => {
            this_transform = Transform::identity();
        },
    }

    let rigid_body_component;
    let collider_component;

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
            
            shape: ColliderShape::cuboid(
                0.208,
                0.277,
                0.213,
            ),
            position: Vec3::new(0., 0.011, -0.004).into(),
            material: ColliderMaterial {
                friction: 0.75,
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
            
            shape: ColliderShape::cuboid(
                0.208,
                0.277,
                0.213,
            ),
            position: Vec3::new(0., 0.011, -0.004).into(),
            material: ColliderMaterial {
                friction: 0.75,
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

    


    

    let mut entity_updates_map = HashMap::new();
    entity_updates_map.insert(".".to_string(), HashMap::new());

    let examine_text = "[font=".to_owned() + FURTHER_NORMAL_FONT + "]*******\n"
    + "A standard issue helmet used by Security Officers."
    + "[font=" + FURTHER_ITALIC_FONT + "]\n\nIt is in perfect shape.[/font]"
    + "\n*******[/font]";
    
    let mut attachment_transforms = HashMap::new();

    attachment_transforms.insert("left_hand".to_string(), Transform::from_matrix(
        Mat4::from_scale_rotation_translation(
        Vec3::new(0.5,0.5,0.5),
      Quat::from_axis_angle(Vec3::new(1.,0.,0.), 3.111607897),
   Vec3::new(0.,-0.003, -0.108)
        )
    ));

    let right_hand_rotation = Vec3::new(0.11473795,0.775676679,0.);
    let right_hand_rotation_length = right_hand_rotation.length();

    attachment_transforms.insert("right_hand".to_string(), Transform::from_matrix(
        Mat4::from_scale_rotation_translation(
        Vec3::new(0.5,0.5,0.5),
      Quat::from_axis_angle(Vec3::new(0.11473795,0.775676679,0.).normalize(), right_hand_rotation_length),
   Vec3::new(0.064,-0.019, 0.065)
        )
    ));

    attachment_transforms.insert("helmet".to_string(), Transform::from_matrix(
        Mat4::from_scale_rotation_translation(
        Vec3::new(0.5,0.5,0.5),
      Quat::from_axis_angle(Vec3::new(1.,0.,0.), -1.41617761),
   Vec3::new(0.,0.132, 0.05)
        )
    ));


    let mut builder = commands.spawn_bundle(rigid_body_component);

    let entity_id = builder.id();
    
    builder.insert_bundle(
        collider_component,
    ).insert_bundle((
        EntityData {
            entity_class : "entity".to_string(),
            entity_type : "helmetSecurity".to_string(),
            entity_group: EntityGroup::None
        },
        EntityUpdates{
            updates: entity_updates_map,
            changed_parameters: vec![],
            excluded_handles:HashMap::new(),
            updates_difference: HashMap::new(),
        },
        WorldMode {
            mode : WorldModes::Physics
        },
        CachedBroadcastTransform::new(),
        Examinable {
            description: examine_text,
            name: "a security helmet".to_string()
        },
        Helmet,
        InventoryItem {
            in_inventory_of_entity: holder_entity_option,
            attachment_transforms: attachment_transforms,
            drop_transform: Transform::from_matrix(
             Mat4::from_scale_rotation_translation(
        Vec3::new(1.,1.,1.),
      Quat::from_axis_angle(Vec3::new(-0.0394818427,0.00003351599,1.), 3.124470974),
   Vec3::new(0.,0.355, 0.)
            ),),
            slot_type: SlotType::Helmet,
            is_attached_when_worn : true,
        },
    ));

    if showcase_instance {
        let handle = showcase_handle_option.unwrap();
        builder.insert_bundle((
            Showcase {
                handle: handle,
            },
        ));
        let entity_updates = HashMap::new();
        net_showcase.as_deref_mut().unwrap().send(NetShowcase{
            handle: handle,
            message: ReliableServerMessage::LoadEntity(
                "entity".to_string(),
                "humanMale".to_string(),
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
            Sensable{
                is_audible : false,
                is_light:false,
                sensed_by_cached:vec![],
                sensed_by:vec![],
                always_sensed : false
            },
        ));
    }

    match held {
        true => {
            builder.insert(RigidBodyDisabled);
        },
        false => {

        },
    }

    match holder_entity_option {
        Some(holder_entity) => {
            builder.insert(RigidBodyLinkTransform{
                follow_entity: holder_entity,
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
