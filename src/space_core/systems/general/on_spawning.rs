use bevy::{math::{Vec2, Vec3}, prelude::{Added, Commands, Entity, EventWriter, Query, ResMut}};
use bevy_rapier3d::prelude::{CoefficientCombineRule, ColliderBundle, ColliderFlags, ColliderMassProps, ColliderMaterial, ColliderShape, ColliderType, InteractionGroups, RigidBodyBundle, RigidBodyCcd, RigidBodyForces, RigidBodyMassPropsFlags, RigidBodyType};

use std::collections::HashMap;

use crate::space_core::{components::{cached_broadcast_transform::CachedBroadcastTransform, connected_player::ConnectedPlayer, entity_data::{EntityData, EntityGroup}, entity_updates::EntityUpdates, examinable::Examinable, inventory::{Inventory, Slot, SlotType}, pawn::Pawn, persistent_player_data::PersistentPlayerData, player_input::PlayerInput, radio::{Radio, RadioChannel}, sensable::Sensable, space_access::SpaceAccess, spawning::Spawning, standard_character::{StandardCharacter, State as HumanState}, visible_checker::VisibleChecker, world_mode::{WorldMode,WorldModes}}, enums::{space_access_enum::SpaceAccessEnum, space_jobs::SpaceJobsEnum}, events::net::{ net_on_spawning::NetOnSpawning}, functions::{collider_interaction_groups::{ColliderGroup, get_bit_masks}, new_chat_message::{FURTHER_ITALIC_FONT, FURTHER_NORMAL_FONT}, transform_to_isometry::transform_to_isometry}, resources::handle_to_entity::HandleToEntity, structs::network_messages::{ReliableServerMessage, ServerConfigMessage}};


pub fn on_spawning(
    mut net_on_new_player_connection : EventWriter<NetOnSpawning>,
    query : Query<(Entity, &Spawning, &ConnectedPlayer, &PersistentPlayerData),Added<Spawning>>,
    
    mut commands : Commands,
    mut handle_to_entity : ResMut<HandleToEntity>,
    
) {
    
    for (
        entity_id,
        spawning_component,
        connected_player_component,
        persistent_player_data_component,
    ) in query.iter() {

        let rigid_body_component = RigidBodyBundle {
            body_type: RigidBodyType::Dynamic,
            position: transform_to_isometry(spawning_component.transform).into(),
            forces : RigidBodyForces {
                gravity_scale: 1.,
                ..Default::default()
            },
            ccd: RigidBodyCcd {
                ccd_enabled: false,
                ..Default::default()
            },
            mass_properties: (RigidBodyMassPropsFlags::ROTATION_LOCKED_X | RigidBodyMassPropsFlags::ROTATION_LOCKED_Y| RigidBodyMassPropsFlags::ROTATION_LOCKED_Z).into(),
            ..Default::default()
        };


        let r = 0.25;
        let masks = get_bit_masks(ColliderGroup::Standard);

        let collider_component = ColliderBundle {
            
            shape: ColliderShape::capsule(
                Vec3::new(0.0,0.0+r,0.0).into(),
                Vec3::new(0.0,1.8-r,0.0).into(),
                r
            ),
            position: Vec3::new(0., 0., 0.).into(),
            collider_type: ColliderType::Solid,
            mass_properties: ColliderMassProps::Density(1.0),
            material: ColliderMaterial {
                friction: 0.0,
                friction_combine_rule:  CoefficientCombineRule::Average,
                ..Default::default()
            },
            flags: ColliderFlags {
                collision_groups: InteractionGroups::new(masks.0,masks.1),
                ..Default::default()
            },
            ..Default::default()
        };

        let mut entity_updates_map = HashMap::new();
        entity_updates_map.insert(".".to_string(), HashMap::new());

        let examine_text = "[font=".to_owned() + FURTHER_NORMAL_FONT + "]*******\n"
        + &persistent_player_data_component.character_name + ", a Security Officer.\n"
        + "He is human.\n"
        + "[font=" + FURTHER_ITALIC_FONT + "]\nHe is in perfect shape.[/font]"
        + "\n*******[/font]";

        let new_entity = commands.spawn_bundle(rigid_body_component).insert_bundle(
            collider_component,
        ).insert_bundle((
            ConnectedPlayer {
                handle: connected_player_component.handle,
                authid: connected_player_component.authid,
            },
            PersistentPlayerData {
                character_name: persistent_player_data_component.character_name.clone(),
            },
            Sensable{
                is_audible : false,
                is_light:false,
                sensed_by_cached:vec![],
                sensed_by:vec![],
                always_sensed : false
            },
            VisibleChecker,
            EntityData {
                entity_class : "entity".to_string(),
                entity_type : "humanMale".to_string(),
                entity_group: EntityGroup::Pawn
            },
            EntityUpdates{
                updates: entity_updates_map,
                changed_parameters: vec![],
                excluded_handles:HashMap::new(),
                updates_difference: HashMap::new(),
            },
            WorldMode {
                mode : WorldModes::Kinematic
            },
            PlayerInput{
                movement_vector : Vec2::ZERO,
                sprinting : false
            },
            StandardCharacter {
                current_animation_state : HumanState::Idle
            },
            Pawn {
                name: persistent_player_data_component.character_name.clone(),
                job: SpaceJobsEnum::Security
            },
            SpaceAccess{
                access : vec![SpaceAccessEnum::Security]
            },
            CachedBroadcastTransform::new(),
            Radio {
                listen_access: vec![RadioChannel::Common, RadioChannel::Security],
                speak_access: vec![RadioChannel::Common, RadioChannel::Security],
            },
            Examinable {
                text: examine_text,
            },
            Inventory {
                slots: vec![
                    Slot {
                        slot_type: SlotType::Generic,
                        slot_name: "left_hand".to_string(),
                        slot_item: None,
                        slot_attachment: Some("Smoothing/pawn/humanMale/rig/leftHand/Position3D".to_string()),
                    },
                    Slot {
                        slot_type: SlotType::Generic,
                        slot_name: "right_hand".to_string(),
                        slot_item: None,
                        slot_attachment: Some("Smoothing/pawn/humanMale/rig/rightHand/Position3D".to_string()),
                    },
                    Slot {
                        slot_type: SlotType::Helmet,
                        slot_name: "helmet".to_string(),
                        slot_item: None,
                        slot_attachment: Some("Smoothing/pawn/humanMale/rig/head/Position3D".to_string()),
                    },
                    Slot {
                        slot_type: SlotType::Jumpsuit,
                        slot_name: "jumpsuit".to_string(),
                        slot_item: None,
                        slot_attachment: Some("Smoothing/pawn/humanMale/rig/humanMale".to_string()),
                    },
                ],
                pickup_slot: "left_hand".to_string(),
            },
        )).id();


        let handle = *handle_to_entity.inv_map.get(&entity_id.id()).unwrap();

        handle_to_entity.inv_map.remove(&entity_id.id());
        handle_to_entity.inv_map.insert(new_entity.id(), handle);

        handle_to_entity.map.remove(&handle);
        handle_to_entity.map.insert(handle, new_entity);

        commands.entity(entity_id).despawn();
        
        net_on_new_player_connection.send(NetOnSpawning{
            handle: handle,
            message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::EntityId(new_entity.id(), new_entity.generation()))
        });


    }

    
    
}
