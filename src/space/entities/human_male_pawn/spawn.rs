use std::collections::HashMap;

use bevy_ecs::{entity::Entity, system::Commands};
use bevy_math::Vec3;
use bevy_rapier3d::prelude::{
    CoefficientCombineRule, ColliderBundle, ColliderFlags, ColliderMaterial, ColliderShape,
    ColliderType, InteractionGroups, RigidBodyBundle, RigidBodyDominance, RigidBodyMassPropsFlags,
    RigidBodyType,
};
use bevy_transform::components::Transform;

use crate::space::core::{
    data_link::components::{DataLink, DataLinkType},
    entity::{
        components::{
            EntityData, EntityGroup, EntityUpdates, Examinable, RichName, Sensable, Showcase,
        },
        events::NetShowcase,
        functions::{
            spawn_entity::spawn_held_entity, transform_to_isometry::transform_to_isometry,
        },
        resources::{SpawnHeldData, SpawnPawnData},
    },
    health::components::{Health, HealthContainer, HumanoidHealth},
    inventory::components::{Inventory, Slot, SlotType},
    map::components::Map,
    networking::resources::{ConsoleCommandVariantValues, ReliableServerMessage},
    pawn::{
        components::{
            ConnectedPlayer, Pawn, PersistentPlayerData, PlayerInput, Radio, RadioChannel, Senser,
            SpaceAccess, SpaceAccessEnum, SpaceJobsEnum, StandardCharacter,
        },
        functions::{get_tab_action::get_tab_action, name_generator::get_dummy_name},
        systems::on_setupui::ENTITY_SPAWN_PARENT,
    },
    physics::{
        components::{WorldMode, WorldModes},
        functions::{get_bit_masks, ColliderGroup},
    },
    rigid_body::components::{CachedBroadcastTransform, DefaultTransform, RigidBodyData},
};

pub struct HumanMalePawnBundle;

pub const CHARACTER_FLOOR_FRICTION: f32 = 7.2;

impl HumanMalePawnBundle {
    pub fn spawn(
        passed_transform: Transform,
        commands: &mut Commands,
        correct_transform: bool,
        pawn_data_option: Option<SpawnPawnData>,
        _held_data_option: Option<SpawnHeldData>,
        _default_map_spawn: bool,
        _properties: HashMap<String, ConsoleCommandVariantValues>,
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
            entity_data,
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

        let r = 0.5;

        this_transform.translation.y = 0.9 - r;

        let friction = CHARACTER_FLOOR_FRICTION;
        let friction_combine_rule = CoefficientCombineRule::Min;

        let rigid_body_component = RigidBodyBundle {
            body_type: RigidBodyType::Dynamic.into(),
            position: transform_to_isometry(this_transform).into(),
            mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
            dominance: RigidBodyDominance(10).into(),
            ..Default::default()
        };

        let masks = get_bit_masks(ColliderGroup::Standard);

        let collider_component = ColliderBundle {
            shape: ColliderShape::capsule(
                Vec3::new(0.0, 0.0 + r, 0.0).into(),
                Vec3::new(0.0, 1.8 - r, 0.0).into(),
                r,
            )
            .into(),
            position: Vec3::ZERO.into(),
            collider_type: ColliderType::Solid.into(),
            material: ColliderMaterial {
                friction,
                friction_combine_rule,
                ..Default::default()
            }
            .into(),
            flags: ColliderFlags {
                collision_groups: InteractionGroups::new(masks.0, masks.1),
                ..Default::default()
            }
            .into(),
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
            }
            None => {
                user_name = "unknown".to_string();
            }
        }
        if !showcase_instance {
            entity_builder.insert_bundle(collider_component);
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
            Health {
                health_container: HealthContainer::Humanoid(HumanoidHealth::default()),
                is_combat_obstacle: true,
                ..Default::default()
            },
            RigidBodyData {
                friction,
                friction_combine_rule,
            },
        ));

        let human_male_entity = entity_builder.id();

        let mut slot_entities: HashMap<String, Entity> = HashMap::new();

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

        let mut entity_commands = commands.entity(human_male_entity);

        entity_commands.insert_bundle((inventory_component, examinable_component));

        if showcase_instance {
            entity_commands.insert(Showcase {
                handle: connected_player_component.unwrap().handle,
            });
            let entity_updates = HashMap::new();
            net_showcase.unwrap().send(NetShowcase {
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
                ),
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
                SpaceAccess {
                    access: vec![SpaceAccessEnum::Security],
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
                    DataLink {
                        links: vec![DataLinkType::FullAtmospherics],
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
        }

        human_male_entity
    }
}
