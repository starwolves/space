use std::collections::HashMap;

use bevy::{math::{Vec3}, prelude::{Commands, Entity, Query, Transform}};
use bevy_rapier3d::prelude::{CoefficientCombineRule, ColliderBundle, ColliderFlags, ColliderMaterial, ColliderShape, ColliderType, InteractionGroups, RigidBodyBundle, RigidBodyDominance, RigidBodyType, ColliderMassProps};

use crate::space_core::{components::{cached_broadcast_transform::CachedBroadcastTransform, connected_player::ConnectedPlayer, default_transform::DefaultTransform, entity_data::{EntityData, EntityGroup}, entity_updates::EntityUpdates, examinable::{Examinable, RichName}, health::{Health, HealthContainer, HumanoidHealth}, interpolation_priority::{InterpolationPriority, InterpolationPriorityStatus}, inventory::{Inventory, Slot, SlotType}, pawn::{Pawn, SpaceAccessEnum, SpaceJobsEnum}, persistent_player_data::PersistentPlayerData, player_input::PlayerInput, radio::{Radio, RadioChannel}, sensable::Sensable, senser::{Senser}, showcase::Showcase, space_access::SpaceAccess, standard_character::{StandardCharacter}, world_mode::{WorldMode, WorldModes}}, events::net::net_showcase::NetShowcase, functions::{converters::transform_to_isometry::transform_to_isometry, entity::{collider_interaction_groups::{ColliderGroup, get_bit_masks}, name_generator::{get_dummy_name}, new_chat_message::{ASTRIX, FURTHER_ITALIC_FONT, FURTHER_NORMAL_FONT, HEALTHY_COLOR, UNHEALTHY_COLOR}, spawn_entity::spawn_held_entity, get_tab_action::get_tab_action}}, resources::{entity_data_resource::{SpawnHeldData, SpawnPawnData}, network_messages::ReliableServerMessage}, systems::general::on_setupui::ENTITY_SPAWN_PARENT};

pub struct HumanMalePawnBundle;

pub const CHARACTER_FLOOR_FRICTION : f32 = 7.2;

impl HumanMalePawnBundle {

    pub fn spawn(
        passed_transform : Transform,
        commands : &mut Commands,
        correct_transform : bool,
        pawn_data_option : Option<SpawnPawnData>,
        _held_data_option : Option<SpawnHeldData>,
    ) -> Entity {

        

        let (
            persistent_player_data_component,
            connected_player_component,
            passed_inventory_setup,
            showcase_instance,
            dummy_instance,
            used_names,
            mut net_showcase,
            default_user_name_option,
            entity_data
        ) = pawn_data_option.unwrap().data;

        let default_transform = Transform::identity();

        let mut this_transform = passed_transform;

        if correct_transform {

            this_transform.rotation = default_transform.rotation;

        }

        let character_name;

        if dummy_instance {

            character_name = get_dummy_name(used_names.unwrap());

        } else {

            character_name = persistent_player_data_component.character_name.clone();

        }
        
        
        let rigid_body_component = RigidBodyBundle {
            body_type: RigidBodyType::Dynamic.into(),
            position: transform_to_isometry(passed_transform).into(),
            //mass_properties: (RigidBodyMassPropsFlags::ROTATION_LOCKED_X | RigidBodyMassPropsFlags::ROTATION_LOCKED_Y| RigidBodyMassPropsFlags::ROTATION_LOCKED_Z).into(),
            dominance: RigidBodyDominance(10).into(),
            ..Default::default()
        };
        

        let r = 0.5;
        let masks = get_bit_masks(ColliderGroup::Standard);

        let collider_component = ColliderBundle {
            
            shape: ColliderShape::capsule(
                Vec3::new(0.0,0.0+r,0.0).into(),
                Vec3::new(0.0,1.8-r,0.0).into(),
                r
            ).into(),
            position: Vec3::ZERO.into(),
            collider_type: ColliderType::Solid.into(),
            material: ColliderMaterial {
                friction: CHARACTER_FLOOR_FRICTION,
                friction_combine_rule:  CoefficientCombineRule::Min,
                ..Default::default()
            }.into(),
            flags: ColliderFlags {
                collision_groups: InteractionGroups::new(masks.0,masks.1),
                ..Default::default()
            }.into(),
            mass_properties: ColliderMassProps::Density(1.0).into(),
            ..Default::default()
        };

        
        let mut entity_builder;

        if !showcase_instance {
            entity_builder = commands.spawn_bundle(rigid_body_component);
        } else {
            entity_builder = commands.spawn();
        }

        let user_name;

        match default_user_name_option {
            Some(name) => {
                user_name = name;
            },
            None => {
                user_name = "unknown".to_string();
            },
        }
        if !showcase_instance {
            entity_builder.insert_bundle(collider_component);
        }
        
        entity_builder.insert_bundle((
            EntityData {
                entity_class : "entity".to_string(),
                entity_type : "humanMale".to_string(),
                entity_group: EntityGroup::Pawn
            },
            EntityUpdates::default(),
            WorldMode {
                mode : WorldModes::Kinematic
            },
            StandardCharacter {
                character_name: character_name.clone(),
                ..Default::default()
            },
            CachedBroadcastTransform::default(),
            PersistentPlayerData {
                character_name: character_name.clone(),
                user_name,
                ..Default::default()
            },
            DefaultTransform::default(),
            InterpolationPriority {
                priority: InterpolationPriorityStatus::High,
            },
            Health {
                health_container :HealthContainer::Humanoid(HumanoidHealth::default()),
                is_combat_obstacle: true,
                ..Default::default()
            }
        ));

        let human_male_entity =  entity_builder.id();

        let mut slot_entities : HashMap<String, Entity>= HashMap::new();

        for (slot_name, item_name) in passed_inventory_setup.iter() {

            let entity_option;

            if showcase_instance {
                entity_option = spawn_held_entity(
                    item_name.to_string(),
                    commands, 
                    human_male_entity,
                    showcase_instance,
                    Some(connected_player_component.unwrap().handle),
                    &mut net_showcase,
                    entity_data,
                );
            } else {
                entity_option = spawn_held_entity(
                    item_name.to_string(),
                    commands, 
                    human_male_entity,
                    showcase_instance,
                    None,
                    &mut net_showcase,
                    entity_data,
                );
            }

            match entity_option {
                Some(entity) => {
                    slot_entities.insert(slot_name.to_string(),entity);
                },
                None => {},
            }

        }

        let left_hand_item;
        match slot_entities.get(&"left_hand".to_string()) {
            Some(entity) => {
                left_hand_item = Some(*entity);
            },
            None => {
                left_hand_item = None;
            },
        }
        let right_hand_item;
        match slot_entities.get(&"right_hand".to_string()) {
            Some(entity) => {
                right_hand_item = Some(*entity);
            },
            None => {
                right_hand_item = None;
            },
        }
        let helmet_hand_item;
        match slot_entities.get(&"helmet".to_string()) {
            Some(entity) => {
                helmet_hand_item = Some(*entity);
            },
            None => {
                helmet_hand_item = None;
            },
        }
        let jumpsuit_hand_item;
        match slot_entities.get(&"jumpsuit".to_string()) {
            Some(entity) => {
                jumpsuit_hand_item = Some(*entity);
            },
            None => {
                jumpsuit_hand_item = None;
            },
        }
        let holster_hand_item;
        match slot_entities.get(&"holster".to_string()) {
            Some(entity) => {
                holster_hand_item = Some(*entity);
            },
            None => {
                holster_hand_item = None;
            },
        }

        let inventory_component = Inventory {
            slots: vec![
                Slot {
                    slot_type: SlotType::Generic,
                    slot_name: "left_hand".to_string(),
                    slot_item: left_hand_item,
                    slot_attachment: Some("Smoothing/pawn/humanMale/rig/leftHand/Position3D".to_string()),
                },
                Slot {
                    slot_type: SlotType::Generic,
                    slot_name: "right_hand".to_string(),
                    slot_item: right_hand_item,
                    slot_attachment: Some("Smoothing/pawn/humanMale/rig/rightHand/Position3D".to_string()),
                },
                Slot {
                    slot_type: SlotType::Helmet,
                    slot_name: "helmet".to_string(),
                    slot_item: helmet_hand_item,
                    slot_attachment: Some("Smoothing/pawn/humanMale/rig/head/Position3D".to_string()),
                },
                Slot {
                    slot_type: SlotType::Jumpsuit,
                    slot_name: "jumpsuit".to_string(),
                    slot_item: jumpsuit_hand_item,
                    slot_attachment: Some("Smoothing/pawn/humanMale/rig/humanMale".to_string()),
                },
                Slot {
                    slot_type: SlotType::Holster,
                    slot_name: "holster".to_string(),
                    slot_item: holster_hand_item,
                    slot_attachment: Some("Smoothing/pawn/humanMale/rig/holster/Position3D".to_string()),
                },
            ],
            active_slot: "left_hand".to_string(),
            ..Default::default()
        };


        let examinable_component = Examinable {            
            name : RichName {
                name: character_name.clone(),
                n: false,
                ..Default::default()
            },
            ..Default::default()
        };

        



        let mut entity_commands = commands.entity(human_male_entity);
        
        entity_commands.insert_bundle((inventory_component,examinable_component));

        if showcase_instance {
            entity_commands.insert(
                Showcase {
                    handle: connected_player_component.unwrap().handle,
                }
            );
            let entity_updates = HashMap::new();
            net_showcase.unwrap().send(NetShowcase{
                handle: connected_player_component.unwrap().handle,
                message: ReliableServerMessage::LoadEntity(
                    "entity".to_string(),
                    "humanMale".to_string(),
                    entity_updates,
                    human_male_entity.to_bits(),
                    true,
                    "main".to_string(),
                    ENTITY_SPAWN_PARENT.to_string(),
                    false,
                )
            });
        } else {

            let mut pawn_component = Pawn {
                name: character_name.clone(),
                job: SpaceJobsEnum::Security,
                ..Default::default()
            };

            // Add default "examine" tab action.
            pawn_component.tab_actions_add("examine", None, get_tab_action("examine").unwrap());
            pawn_component.tab_actions_add("pickup", None, get_tab_action("pickup").unwrap());

            entity_commands.insert_bundle((
                Senser::default(),
                Sensable::default(),
                Radio {
                    listen_access: vec![RadioChannel::Common, RadioChannel::Security],
                    speak_access: vec![RadioChannel::Common, RadioChannel::Security],
                },
                SpaceAccess{
                    access : vec![SpaceAccessEnum::Security]
                },
                pawn_component,
                PlayerInput::default(),
            ));

            if !dummy_instance {

                entity_commands.insert_bundle((
                    ConnectedPlayer {
                        handle: connected_player_component.unwrap().handle,
                        authid: connected_player_component.unwrap().authid,
                        ..Default::default()
                    },
                ));

            }

        }

        human_male_entity

    }

}

pub fn generate_human_examine_text(
    character_name : &str,
    inventory_component_option : Option<&Inventory>,
    examinables : &Query<&Examinable>,
    health_component : &Health,
) -> String {

    let mut examine_text = "[font=".to_owned() + FURTHER_NORMAL_FONT + "]" + ASTRIX + "\n"
    + character_name + ", a Security Officer.\n"
    + "He is human.\n";


    match &health_component.health_container {
        crate::space_core::components::health::HealthContainer::Humanoid(humanoid_container) => {

            let head_damage = humanoid_container.head_brute+humanoid_container.head_burn+humanoid_container.head_toxin;
            let torso_damage = humanoid_container.torso_brute+humanoid_container.torso_burn+humanoid_container.torso_toxin;
            let left_arm_damage = humanoid_container.left_arm_brute+humanoid_container.left_arm_burn+humanoid_container.left_arm_toxin;
            let right_arm_damage = humanoid_container.right_arm_brute+humanoid_container.right_arm_burn+humanoid_container.right_arm_toxin;
            let left_leg_damage = humanoid_container.left_leg_brute+humanoid_container.left_leg_burn+humanoid_container.left_leg_toxin;
            let right_leg_damage = humanoid_container.right_leg_brute+humanoid_container.right_leg_burn+humanoid_container.right_leg_toxin;

            if head_damage < 25. && torso_damage < 25. && left_arm_damage < 25. && right_arm_damage < 25. && left_leg_damage < 25. && right_leg_damage < 25. {

                examine_text = examine_text + "[font=" + FURTHER_ITALIC_FONT + "][color=" + HEALTHY_COLOR + "]He is in perfect shape.[/color][/font]\n";

            } else {

                if humanoid_container.head_brute > 75. {
                    examine_text = examine_text + "[font=" + FURTHER_ITALIC_FONT + "][color=" + UNHEALTHY_COLOR + "]His head is heavily injured.[/color][/font]\n";
                } else if humanoid_container.head_brute > 50. {
                    examine_text = examine_text + "[font=" + FURTHER_ITALIC_FONT + "][color=" + UNHEALTHY_COLOR + "]His head is injured.[/color][/font]\n";
                } else if humanoid_container.head_brute > 25. {
                    examine_text = examine_text + "[font=" + FURTHER_ITALIC_FONT + "][color=" + UNHEALTHY_COLOR + "]His head is bruised.[/color][/font]\n";
                }


                if humanoid_container.torso_brute > 75. {
                    examine_text = examine_text + "[font=" + FURTHER_ITALIC_FONT + "][color=" + UNHEALTHY_COLOR + "]His torso is heavily injured.[/color][/font]\n";
                } else if humanoid_container.torso_brute > 50. {
                    examine_text = examine_text + "[font=" + FURTHER_ITALIC_FONT + "][color=" + UNHEALTHY_COLOR + "]His torso is injured.[/color][/font]\n";
                } else if humanoid_container.torso_brute > 25. {
                    examine_text = examine_text + "[font=" + FURTHER_ITALIC_FONT + "][color=" + UNHEALTHY_COLOR + "]His torso is bruised.[/color][/font]\n";
                }


                if humanoid_container.left_arm_brute > 75. {
                    examine_text = examine_text + "[font=" + FURTHER_ITALIC_FONT + "][color=" + UNHEALTHY_COLOR + "]His left arm is heavily injured.[/color][/font]\n";
                } else if humanoid_container.left_arm_brute > 50. {
                    examine_text = examine_text + "[font=" + FURTHER_ITALIC_FONT + "][color=" + UNHEALTHY_COLOR + "]His left arm is injured.[/color][/font]\n";
                } else if humanoid_container.left_arm_brute > 25. {
                    examine_text = examine_text + "[font=" + FURTHER_ITALIC_FONT + "][color=" + UNHEALTHY_COLOR + "]His left arm is bruised.[/color][/font]\n";
                }


                if humanoid_container.right_arm_brute > 75. {
                    examine_text = examine_text + "[font=" + FURTHER_ITALIC_FONT + "][color=" + UNHEALTHY_COLOR + "]His right arm is heavily injured.[/color][/font]\n";
                } else if humanoid_container.right_arm_brute > 50. {
                    examine_text = examine_text + "[font=" + FURTHER_ITALIC_FONT + "][color=" + UNHEALTHY_COLOR + "]His right arm is injured.[/color][/font]\n";
                } else if humanoid_container.right_arm_brute > 25. {
                    examine_text = examine_text + "[font=" + FURTHER_ITALIC_FONT + "][color=" + UNHEALTHY_COLOR + "]His right arm is bruised.[/color][/font]\n";
                }

                if humanoid_container.left_leg_brute > 75. {
                    examine_text = examine_text + "[font=" + FURTHER_ITALIC_FONT + "][color=" + UNHEALTHY_COLOR + "]His left leg is heavily injured.[/color][/font]\n";
                } else if humanoid_container.left_leg_brute > 50. {
                    examine_text = examine_text + "[font=" + FURTHER_ITALIC_FONT + "][color=" + UNHEALTHY_COLOR + "]His left leg is injured.[/color][/font]\n";
                } else if humanoid_container.left_leg_brute > 25. {
                    examine_text = examine_text + "[font=" + FURTHER_ITALIC_FONT + "][color=" + UNHEALTHY_COLOR + "]His left leg is bruised.[/color][/font]\n";
                }

                if humanoid_container.right_leg_brute > 75. {
                    examine_text = examine_text + "[font=" + FURTHER_ITALIC_FONT + "][color=" + UNHEALTHY_COLOR + "]His right leg is heavily injured.[/color][/font]\n";
                } else if humanoid_container.right_leg_brute > 50. {
                    examine_text = examine_text + "[font=" + FURTHER_ITALIC_FONT + "][color=" + UNHEALTHY_COLOR + "]His right leg is injured.[/color][/font]\n";
                } else if humanoid_container.right_leg_brute > 25. {
                    examine_text = examine_text + "[font=" + FURTHER_ITALIC_FONT + "][color=" + UNHEALTHY_COLOR + "]His right leg is bruised.[/color][/font]\n";
                }


                if humanoid_container.head_burn > 75. {
                    examine_text = examine_text + "[font=" + FURTHER_ITALIC_FONT + "][color=" + UNHEALTHY_COLOR + "]His head has visible third degree burns, ouch![/color][/font]\n";
                } else if humanoid_container.head_burn > 50. {
                    examine_text = examine_text + "[font=" + FURTHER_ITALIC_FONT + "][color=" + UNHEALTHY_COLOR + "]His head has visible second degree burns.[/color][/font]\n";
                } else if humanoid_container.head_burn > 25. {
                    examine_text = examine_text + "[font=" + FURTHER_ITALIC_FONT + "][color=" + UNHEALTHY_COLOR + "]His head has visible first degree burns.[/color][/font]\n";
                }
                
                
                if humanoid_container.torso_burn > 75. {
                    examine_text = examine_text + "[font=" + FURTHER_ITALIC_FONT + "][color=" + UNHEALTHY_COLOR + "]His torso has visible third degree burns, ouch![/color][/font]\n";
                } else if humanoid_container.torso_burn > 50. {
                    examine_text = examine_text + "[font=" + FURTHER_ITALIC_FONT + "][color=" + UNHEALTHY_COLOR + "]His torso has visible second degree burns.[/color][/font]\n";
                } else if humanoid_container.torso_burn > 25. {
                    examine_text = examine_text + "[font=" + FURTHER_ITALIC_FONT + "][color=" + UNHEALTHY_COLOR + "]His torso has visible first degree burns.[/color][/font]\n";
                }
                
                
                if humanoid_container.left_arm_burn > 75. {
                    examine_text = examine_text + "[font=" + FURTHER_ITALIC_FONT + "][color=" + UNHEALTHY_COLOR + "]His left arm has visible third degree burns, ouch![/color][/font]\n";
                } else if humanoid_container.left_arm_burn > 50. {
                    examine_text = examine_text + "[font=" + FURTHER_ITALIC_FONT + "][color=" + UNHEALTHY_COLOR + "]His left arm has visible second degree burns.[/color][/font]\n";
                } else if humanoid_container.left_arm_burn > 25. {
                    examine_text = examine_text + "[font=" + FURTHER_ITALIC_FONT + "][color=" + UNHEALTHY_COLOR + "]His left arm has visible first degree burns.[/color][/font]\n";
                }
                
                
                if humanoid_container.right_arm_burn > 75. {
                    examine_text = examine_text + "[font=" + FURTHER_ITALIC_FONT + "][color=" + UNHEALTHY_COLOR + "]His right arm has visible third degree burns, ouch![/color][/font]\n";
                } else if humanoid_container.right_arm_burn > 50. {
                    examine_text = examine_text + "[font=" + FURTHER_ITALIC_FONT + "][color=" + UNHEALTHY_COLOR + "]His right arm has visible second degree burns.[/color][/font]\n";
                } else if humanoid_container.right_arm_burn > 25. {
                    examine_text = examine_text + "[font=" + FURTHER_ITALIC_FONT + "][color=" + UNHEALTHY_COLOR + "]His right arm has visible first degree burns.[/color][/font]\n";
                }
                
                if humanoid_container.left_leg_burn > 75. {
                    examine_text = examine_text + "[font=" + FURTHER_ITALIC_FONT + "][color=" + UNHEALTHY_COLOR + "]His left leg has visible third degree burns, ouch![/color][/font]\n";
                } else if humanoid_container.left_leg_burn > 50. {
                    examine_text = examine_text + "[font=" + FURTHER_ITALIC_FONT + "][color=" + UNHEALTHY_COLOR + "]His left leg has visible second degree burns.[/color][/font]\n";
                } else if humanoid_container.left_leg_burn > 25. {
                    examine_text = examine_text + "[font=" + FURTHER_ITALIC_FONT + "][color=" + UNHEALTHY_COLOR + "]His left leg has visible first degree burns.[/color][/font]\n";
                }
                
                if humanoid_container.right_leg_burn > 75. {
                    examine_text = examine_text + "[font=" + FURTHER_ITALIC_FONT + "][color=" + UNHEALTHY_COLOR + "]His right leg has visible third degree burns, ouch![/color][/font]\n";
                } else if humanoid_container.right_leg_burn > 50. {
                    examine_text = examine_text + "[font=" + FURTHER_ITALIC_FONT + "][color=" + UNHEALTHY_COLOR + "]His right leg has visible second degree burns.[/color][/font]\n";
                } else if humanoid_container.right_leg_burn > 25. {
                    examine_text = examine_text + "[font=" + FURTHER_ITALIC_FONT + "][color=" + UNHEALTHY_COLOR + "]His right leg has visible first degree burns.[/color][/font]\n";
                }
                

            }

        },
        _=>(),
    }

    

    match inventory_component_option {
        Some(inventory_component) => {
            examine_text = examine_text + "\n";
            for slot in inventory_component.slots.iter() {
                match slot.slot_item {
                    Some(slot_item_entity) => {

                        let examinable = examinables.get(slot_item_entity)
                        .expect("inventory_update.rs::generate_human_examine_text couldn't find inventory_item_component of an item from passed inventory.");

                        if slot.slot_name == "left_hand"  {
                            examine_text = examine_text + "He is holding " + &examinable.name.get_a_name() + " in his left hand.\n";
                        } else if slot.slot_name == "right_hand" {
                            examine_text = examine_text + "He is holding " + &examinable.name.get_a_name() + " in his right hand.\n";
                        } else if slot.slot_name == "helmet" {
                            examine_text = examine_text + "He is wearing " + &examinable.name.get_a_name() + " on his head.\n";
                        } else if slot.slot_name == "jumpsuit" {
                            examine_text = examine_text + "He is wearing " + &examinable.name.get_a_name() + " on his body.\n";
                        } else if slot.slot_name == "holster" {
                            examine_text = examine_text + &examinable.name.get_a_name() + "is attached to his holster.\n";
                        } else {
                            examine_text = examine_text + "He is wearing " + &examinable.name.get_a_name() + ".\n";
                        }

                    },
                    None => {},
                }
            }
        },
        None => {},
    }

    examine_text = examine_text + ASTRIX + "[/font]";

    examine_text

}
