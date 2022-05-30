pub mod entity_bundle;
pub mod rigidbody_bundle;

use std::collections::HashMap;

use bevy_ecs::entity::Entity;
use bevy_rapier3d::prelude::{Dominance, LockedAxes};
use bevy_transform::prelude::Transform;

use crate::core::{
    chat::components::{Radio, RadioChannel},
    connected_player::{components::ConnectedPlayer, functions::name_generator::get_dummy_name},
    data_link::components::{DataLink, DataLinkType},
    entity::{
        functions::spawn_entity::spawn_held_entity,
        resources::{PawnDesignation, SpawnData},
        spawn::{base_entity_builder, showcase_builder, BaseEntityData, ShowCaseBuilderData},
    },
    health::components::{Health, HealthContainer, HumanoidHealth},
    humanoid::components::Humanoid,
    inventory::components::{Inventory, Slot, SlotType},
    map::components::Map,
    pawn::components::{
        ControllerInput, Pawn, PersistentPlayerData, ShipAuthorization, ShipAuthorizationEnum,
        ShipJobsEnum,
    },
    physics::components::{WorldMode, WorldModes},
    rigid_body::spawn::{rigidbody_builder, RigidBodySpawnData},
    senser::components::Senser,
    tab_actions::functions::get_tab_action,
};

use entity_bundle::entity_bundle;
use rigidbody_bundle::rigidbody_bundle;

use self::rigidbody_bundle::R;

pub const CHARACTER_FLOOR_FRICTION: f32 = 7.2;

pub struct HumanMaleBundle;

impl HumanMaleBundle {
    pub fn spawn(mut spawn_data: SpawnData) -> Entity {
        let entity = spawn_data.commands.spawn().id();

        let default_transform = Transform::identity();

        if spawn_data.correct_transform {
            spawn_data.entity_transform.rotation = default_transform.rotation;
        }

        spawn_data.entity_transform.translation.y = 0.9 - R;

        let (
            persistent_player_data_component,
            connected_player_component,
            passed_inventory_setup,
            pawn_designation,
            used_names,
            default_user_name_option,
            entity_data,
        ) = spawn_data.pawn_data_option.unwrap().data;

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

        let user_name;

        match default_user_name_option {
            Some(name) => {
                user_name = name;
            }
            None => {
                user_name = "unknown".to_string();
            }
        }

        let rigidbody_bundle = rigidbody_bundle();
        let entity_bundle = entity_bundle(default_transform, character_name.clone());

        rigidbody_builder(
            &mut spawn_data.commands,
            entity,
            RigidBodySpawnData {
                rigidbody_dynamic: true,
                rigid_transform: spawn_data.entity_transform,
                entity_is_stored_item: spawn_data.held_data_option.is_some(),
                collider: rigidbody_bundle.collider,
                collider_transform: rigidbody_bundle.collider_transform,
                collider_friction: rigidbody_bundle.collider_friction,

                ..Default::default()
            },
        );

        base_entity_builder(
            &mut spawn_data.commands,
            entity,
            BaseEntityData {
                dynamicbody: true,
                entity_type: entity_bundle.entity_name.clone(),
                examinable: entity_bundle.examinable,
                health: Health {
                    health_container: HealthContainer::Humanoid(HumanoidHealth::default()),
                    is_combat_obstacle: true,
                    ..Default::default()
                },
                is_showcase: spawn_data.showcase_data_option.is_some(),
                ..Default::default()
            },
        );

        showcase_builder(
            &mut spawn_data.commands,
            entity,
            spawn_data.showcase_data_option,
            ShowCaseBuilderData {
                entity_type: entity_bundle.entity_name,
                entity_updates: HashMap::new(),
            },
        );

        let mut spawner = spawn_data.commands.entity(entity);

        if spawn_data.showcase_data_option.is_none() {
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

            spawner.insert_bundle((
                Senser::default(),
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
                    spawner.insert_bundle((
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
                _ => (),
            }
        }

        spawner.insert_bundle((
            Humanoid {
                character_name: character_name.clone(),
                ..Default::default()
            },
            PersistentPlayerData {
                character_name: character_name.clone(),
                user_name,
                ..Default::default()
            },
            WorldMode {
                mode: WorldModes::Kinematic,
            },
        ));

        spawner
            .insert(Dominance::group(10))
            .insert(LockedAxes::ROTATION_LOCKED);

        let mut slot_entities: HashMap<String, Entity> = HashMap::new();

        for (slot_name, item_name) in passed_inventory_setup.iter() {
            let entity_option;

            entity_option = spawn_held_entity(
                item_name.to_string(),
                spawn_data.commands,
                entity,
                &mut spawn_data.showcase_data_option,
                entity_data,
            );

            match entity_option {
                Some(entity) => {
                    slot_entities.insert(slot_name.to_string(), entity);
                }
                None => {}
            }
        }

        let mut spawner = spawn_data.commands.entity(entity);

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

        spawner.insert(Inventory {
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
        });

        entity
    }
}
