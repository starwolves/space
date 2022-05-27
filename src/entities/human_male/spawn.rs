use std::collections::HashMap;

use bevy_ecs::entity::Entity;
use bevy_hierarchy::BuildChildren;
use bevy_math::Vec3;
use bevy_rapier3d::prelude::{
    CoefficientCombineRule, Collider, CollisionGroups, Dominance, ExternalForce, ExternalImpulse,
    Friction, GravityScale, LockedAxes, RigidBody, Sleeping, Velocity,
};
use bevy_transform::components::Transform;

use crate::core::{
    artificial_unintelligence::components::{AiGoal, Blob, Path},
    chat::components::{Radio, RadioChannel},
    connected_player::{
        components::ConnectedPlayer, functions::name_generator::get_dummy_name,
        systems::on_setupui::ENTITY_SPAWN_PARENT,
    },
    data_link::components::{DataLink, DataLinkType},
    entity::{
        components::{EntityData, EntityGroup, EntityUpdates, Showcase},
        events::NetShowcase,
        functions::spawn_entity::spawn_held_entity,
        resources::{PawnDesignation, SpawnData},
    },
    examinable::components::{Examinable, RichName},
    health::components::{Health, HealthContainer, HumanoidHealth},
    humanoid::components::Humanoid,
    inventory::components::{Inventory, Slot, SlotType},
    map::components::Map,
    networking::resources::ReliableServerMessage,
    pawn::components::{
        ControllerInput, Pawn, PersistentPlayerData, ShipAuthorization, ShipAuthorizationEnum,
        ShipJobsEnum,
    },
    physics::{
        components::{WorldMode, WorldModes},
        functions::{get_bit_masks, ColliderGroup},
    },
    rigid_body::components::{CachedBroadcastTransform, DefaultTransform, RigidBodyData},
    sensable::components::Sensable,
    senser::components::Senser,
    tab_actions::functions::get_tab_action,
};

pub struct HumanMaleBundle;

pub const CHARACTER_FLOOR_FRICTION: f32 = 7.2;

impl HumanMaleBundle {
    pub fn spawn(mut spawn_data: SpawnData) -> Entity {
        let (
            persistent_player_data_component,
            connected_player_component,
            passed_inventory_setup,
            pawn_designation,
            used_names,
            default_user_name_option,
            entity_data,
        ) = spawn_data.pawn_data_option.unwrap().data;

        let default_transform = Transform::identity();

        let mut this_transform = spawn_data.entity_transform;

        if spawn_data.correct_transform {
            this_transform.rotation = default_transform.rotation;
        }

        let character_name;

        match pawn_designation {
            PawnDesignation::Dummy => {
                character_name = get_dummy_name(used_names.unwrap());
            }
            PawnDesignation::Ai => {
                character_name = "Ai".to_string();
            }
            _ => {
                character_name = persistent_player_data_component.character_name.clone();
            }
        }

        let r = 0.5;

        this_transform.translation.y = 0.9 - r;

        let friction = CHARACTER_FLOOR_FRICTION;
        let friction_combine_rule = CoefficientCombineRule::Min;

        let masks = get_bit_masks(ColliderGroup::Standard);

        let collider = Collider::capsule(
            Vec3::new(0.0, 0.0 + r, 0.0).into(),
            Vec3::new(0.0, 1.8 - r, 0.0).into(),
            r,
        );

        let mut friction = Friction::coefficient(friction);
        friction.combine_rule = friction_combine_rule;

        let mut entity_builder = spawn_data.commands.spawn();
        let human_male_entity = entity_builder.id();

        match pawn_designation {
            PawnDesignation::Showcase => {
                //Nothing.
            }
            _ => {
                entity_builder
                    .insert(RigidBody::Dynamic)
                    .insert(this_transform)
                    .insert(Velocity::default())
                    .insert(ExternalForce::default())
                    .insert(Dominance::group(10))
                    .insert(LockedAxes::ROTATION_LOCKED)
                    .insert(ExternalImpulse::default())
                    .insert(Sleeping::default())
                    .insert(GravityScale::default())
                    .with_children(|children| {
                        children
                            .spawn()
                            .insert(collider)
                            .insert(CollisionGroups::new(masks.0, masks.1))
                            .insert(friction);
                    });
            }
        }

        let user_name;

        match default_user_name_option {
            Some(name) => {
                user_name = name;
            }
            None => {
                user_name = "unknown".to_string();
            }
        }

        entity_builder.insert_bundle((
            EntityData {
                entity_class: "entity".to_string(),
                entity_name: "humanMale".to_string(),
                entity_group: EntityGroup::Pawn,
            },
            EntityUpdates::default(),
            WorldMode {
                mode: WorldModes::Kinematic,
            },
            Humanoid {
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
            Health {
                health_container: HealthContainer::Humanoid(HumanoidHealth::default()),
                is_combat_obstacle: true,
                ..Default::default()
            },
            RigidBodyData {
                friction: friction.coefficient,
                friction_combine_rule: friction.combine_rule,
            },
        ));

        let mut slot_entities: HashMap<String, Entity> = HashMap::new();

        for (slot_name, item_name) in passed_inventory_setup.iter() {
            let entity_option;

            if let PawnDesignation::Showcase = pawn_designation {
                entity_option = spawn_held_entity(
                    item_name.to_string(),
                    spawn_data.commands,
                    human_male_entity,
                    &mut spawn_data.showcase_data_option,
                    entity_data,
                );
            } else {
                entity_option = spawn_held_entity(
                    item_name.to_string(),
                    spawn_data.commands,
                    human_male_entity,
                    &mut None,
                    entity_data,
                );
            }

            match entity_option {
                Some(entity) => {
                    slot_entities.insert(slot_name.to_string(), entity);
                }
                None => {}
            }
        }

        let left_hand_item;
        match slot_entities.get(&"left_hand".to_string()) {
            Some(entity) => {
                left_hand_item = Some(*entity);
            }
            None => {
                left_hand_item = None;
            }
        }
        let right_hand_item;
        match slot_entities.get(&"right_hand".to_string()) {
            Some(entity) => {
                right_hand_item = Some(*entity);
            }
            None => {
                right_hand_item = None;
            }
        }
        let helmet_hand_item;
        match slot_entities.get(&"helmet".to_string()) {
            Some(entity) => {
                helmet_hand_item = Some(*entity);
            }
            None => {
                helmet_hand_item = None;
            }
        }
        let jumpsuit_hand_item;
        match slot_entities.get(&"jumpsuit".to_string()) {
            Some(entity) => {
                jumpsuit_hand_item = Some(*entity);
            }
            None => {
                jumpsuit_hand_item = None;
            }
        }
        let holster_hand_item;
        match slot_entities.get(&"holster".to_string()) {
            Some(entity) => {
                holster_hand_item = Some(*entity);
            }
            None => {
                holster_hand_item = None;
            }
        }

        let inventory_component = Inventory {
            slots: vec![
                Slot {
                    slot_type: SlotType::Generic,
                    slot_name: "left_hand".to_string(),
                    slot_item: left_hand_item,
                    slot_attachment: Some(
                        "Smoothing/pawn/humanMale/rig/leftHand/Position3D".to_string(),
                    ),
                },
                Slot {
                    slot_type: SlotType::Generic,
                    slot_name: "right_hand".to_string(),
                    slot_item: right_hand_item,
                    slot_attachment: Some(
                        "Smoothing/pawn/humanMale/rig/rightHand/Position3D".to_string(),
                    ),
                },
                Slot {
                    slot_type: SlotType::Helmet,
                    slot_name: "helmet".to_string(),
                    slot_item: helmet_hand_item,
                    slot_attachment: Some(
                        "Smoothing/pawn/humanMale/rig/head/Position3D".to_string(),
                    ),
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
                    slot_attachment: Some(
                        "Smoothing/pawn/humanMale/rig/holster/Position3D".to_string(),
                    ),
                },
            ],
            active_slot: "left_hand".to_string(),
            ..Default::default()
        };

        let examinable_component = Examinable {
            name: RichName {
                name: character_name.clone(),
                n: false,
                ..Default::default()
            },
            ..Default::default()
        };

        let mut entity_commands = spawn_data.commands.entity(human_male_entity);

        entity_commands.insert_bundle((inventory_component, examinable_component));

        if let PawnDesignation::Showcase = pawn_designation {
            entity_commands.insert(Showcase {
                handle: connected_player_component.unwrap().handle,
            });
            let entity_updates = HashMap::new();

            let data = spawn_data.showcase_data_option.as_mut().unwrap();

            data.event_writer.send(NetShowcase {
                handle: data.handle,
                message: ReliableServerMessage::LoadEntity(
                    "entity".to_string(),
                    "humanMale".to_string(),
                    entity_updates,
                    human_male_entity.to_bits(),
                    true,
                    "main".to_string(),
                    ENTITY_SPAWN_PARENT.to_string(),
                    false,
                ),
            });
        } else {
            let mut pawn_component = Pawn {
                name: character_name.clone(),
                job: ShipJobsEnum::Security,
                ..Default::default()
            };

            pawn_component.tab_actions_add(
                "actions::pawn/examine",
                None,
                get_tab_action("actions::pawn/examine").unwrap(),
            );
            pawn_component.tab_actions_add(
                "actions::inventory/pickup",
                None,
                get_tab_action("actions::inventory/pickup").unwrap(),
            );

            entity_commands.insert_bundle((
                Senser::default(),
                Sensable::default(),
                Radio {
                    listen_access: vec![RadioChannel::Common, RadioChannel::Security],
                    speak_access: vec![RadioChannel::Common, RadioChannel::Security],
                },
                ShipAuthorization {
                    access: vec![ShipAuthorizationEnum::Security],
                },
                pawn_component,
                ControllerInput::default(),
            ));

            match pawn_designation {
                PawnDesignation::Player => {
                    entity_commands.insert_bundle((
                        ConnectedPlayer {
                            handle: connected_player_component.unwrap().handle,
                            authid: connected_player_component.unwrap().authid,
                            ..Default::default()
                        },
                        DataLink {
                            links: vec![
                                DataLinkType::FullAtmospherics,
                                DataLinkType::RemoteLock,
                                DataLinkType::ShipEngineeringKnowledge,
                            ],
                        },
                        Map {
                            available_display_modes: vec![
                                ("Standard".to_string(), "standard".to_string()),
                                (
                                    "Atmospherics Liveable".to_string(),
                                    "atmospherics_liveable".to_string(),
                                ),
                                (
                                    "Atmospherics Temperature".to_string(),
                                    "atmospherics_temperature".to_string(),
                                ),
                                (
                                    "Atmospherics Pressure".to_string(),
                                    "atmospherics_pressure".to_string(),
                                ),
                            ],
                            ..Default::default()
                        },
                    ));
                }
                PawnDesignation::Ai => {
                    entity_commands.insert_bundle((
                        AiGoal {
                            ..Default::default()
                        },
                        Blob {
                            ..Default::default()
                        },
                        Path {
                            ..Default::default()
                        },
                    ));
                }
                PawnDesignation::Dummy => {}
                _ => {}
            }
        }

        human_male_entity
    }
}
