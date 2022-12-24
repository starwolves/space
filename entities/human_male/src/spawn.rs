use std::collections::{BTreeMap, HashMap};

use bevy::prelude::{Commands, Entity, EventReader, EventWriter, ResMut, Transform, Vec3};
use bevy_rapier3d::prelude::{CoefficientCombineRule, Collider, Dominance, Friction, LockedAxes};
use chat::chat::{Radio, RadioChannel};
use entity::{
    entity_data::{WorldMode, WorldModes},
    entity_types::{BoxedEntityType, EntityType},
    examine::{Examinable, RichName},
    health::{DamageFlag, Health, HealthContainer, HumanoidHealth},
    meta::EntityDataResource,
    senser::Senser,
    spawn::{
        base_entity_builder, BaseEntityBuilder, BaseEntityBundle, BaseEntityData,
        DefaultSpawnEvent, EntityBuildData, NoData, SpawnEntity,
    },
};
use humanoid::humanoid::{Humanoid, HUMAN_DUMMY_ENTITY_NAME};
use inventory::inventory::{Inventory, Slot, SlotType};
use inventory::{
    combat::{DamageModel, MeleeCombat},
    spawn_item::spawn_held_entity,
};
use map::map::Map;
use pawn::pawn::{DataLink, DataLinkType};
use pawn::pawn::{Pawn, PawnDesignation, ShipAuthorization, ShipAuthorizationEnum, SpawnPawnData};
use physics::physics::CHARACTER_FLOOR_FRICTION;
use physics::spawn::{RigidBodyBuilder, RigidBodyBundle};

/// Get default transform.
#[cfg(any(feature = "server", feature = "client"))]
pub fn get_default_transform() -> Transform {
    Transform::IDENTITY
}

/// Human male spawn data.
#[cfg(any(feature = "server", feature = "client"))]
pub struct HumanMaleBuildData;

#[cfg(any(feature = "server", feature = "client"))]
impl BaseEntityBuilder<HumanMaleBuildData> for HumanMaleType {
    fn get_bundle(
        &self,
        _spawn_data: &EntityBuildData,
        mut _entity_data: HumanMaleBuildData,
    ) -> BaseEntityBundle {
        let character_name;

        match self.spawn_pawn_data.designation {
            PawnDesignation::Dummy => {
                character_name = "dummy".to_string();
            }
            PawnDesignation::Ai => {
                character_name = "Ai".to_string();
            }
            _ => {
                character_name = self.spawn_pawn_data.pawn_component.character_name.clone();
            }
        }

        let examine_map = BTreeMap::new();
        BaseEntityBundle {
            default_transform: get_default_transform(),
            examinable: Examinable {
                assigned_texts: examine_map,
                name: RichName {
                    name: character_name.clone(),
                    n: false,
                    ..Default::default()
                },
                ..Default::default()
            },
            entity_type: Box::new(HumanMaleType::new()),
            health: Health {
                health_container: HealthContainer::Humanoid(HumanoidHealth::default()),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}
use networking::server::OutgoingReliableServerMessage;

use bevy::prelude::Res;
use entity::entity_types::EntityTypes;
use entity::net::EntityServerMessage;
/// Human male spawner.
#[cfg(any(feature = "server", feature = "client"))]
pub fn build_base_human_males<T: BaseEntityBuilder<HumanMaleBuildData> + 'static>(
    mut spawn_events: EventReader<SpawnEntity<T>>,
    mut commands: Commands,
    mut server: EventWriter<OutgoingReliableServerMessage<EntityServerMessage>>,
    types: Res<EntityTypes>,
) {
    for spawn_event in spawn_events.iter() {
        let base_entity_bundle = spawn_event
            .builder
            .get_bundle(&spawn_event.spawn_data, HumanMaleBuildData);
        let entity_type = base_entity_bundle.entity_type.to_string();
        base_entity_builder(
            &mut commands,
            BaseEntityData {
                entity_type: base_entity_bundle.entity_type,
                examinable: base_entity_bundle.examinable,
                health: base_entity_bundle.health,
                entity_group: base_entity_bundle.entity_group,
                default_map_spawn: base_entity_bundle.default_map_spawn,
                showcase_handle_option: spawn_event.spawn_data.showcase_data_option.clone(),
                ..Default::default()
            },
            spawn_event.spawn_data.entity,
        );

        match &spawn_event.spawn_data.showcase_data_option {
            Some(showcase_data) => {
                server.send(OutgoingReliableServerMessage {
                    handle: showcase_data.handle,
                    message: EntityServerMessage::LoadEntity(
                        *types.netcode_types.get(&entity_type).unwrap(),
                        spawn_event.spawn_data.entity.to_bits(),
                    ),
                });
            }
            None => {}
        }
    }
}

/// Human male spawner.
#[cfg(any(feature = "server", feature = "client"))]
#[derive(Default, Clone)]
pub struct HumanMaleType {
    pub identifier: String,
    pub spawn_pawn_data: SpawnPawnData,
}
impl EntityType for HumanMaleType {
    fn to_string(&self) -> String {
        self.identifier.clone()
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        HumanMaleType::default()
    }
    fn is_type(&self, other_type: BoxedEntityType) -> bool {
        other_type.to_string() == self.identifier
    }
}
#[cfg(any(feature = "server", feature = "client"))]
pub const R: f32 = 0.5;

#[cfg(any(feature = "server", feature = "client"))]
impl RigidBodyBuilder<NoData> for HumanMaleType {
    fn get_bundle(&self, _spawn_data: &EntityBuildData, _entity_data: NoData) -> RigidBodyBundle {
        let mut friction = Friction::coefficient(CHARACTER_FLOOR_FRICTION);
        friction.combine_rule = CoefficientCombineRule::Min;

        RigidBodyBundle {
            collider: Collider::capsule(
                Vec3::new(0.0, 0.0 + R, 0.0).into(),
                Vec3::new(0.0, 1.8 - R, 0.0).into(),
                R,
            ),
            collider_transform: Transform::from_translation(Vec3::new(0., 0.011, -0.004)),
            collider_friction: friction,
            rigidbody_dynamic: true,
            ..Default::default()
        }
    }
}

#[cfg(any(feature = "server", feature = "client"))]
impl HumanMaleBuilder for HumanMaleType {
    fn get_spawn_pawn_data(&self) -> SpawnPawnData {
        self.spawn_pawn_data.clone()
    }
}

#[cfg(any(feature = "server", feature = "client"))]
pub trait HumanMaleBuilder: Send + Sync {
    fn get_spawn_pawn_data(&self) -> SpawnPawnData;
}
use controller::controller::ControllerInput;

/// human-male specific spawn components and bundles.
#[cfg(any(feature = "server", feature = "client"))]
pub fn build_human_males<T: HumanMaleBuilder + 'static>(
    mut commands: Commands,
    mut spawn_events: EventReader<SpawnEntity<T>>,
    mut default_spawner: EventWriter<DefaultSpawnEvent>,
    entity_data: ResMut<EntityDataResource>,
) {
    for spawn_event in spawn_events.iter() {
        let mut spawner = commands.entity(spawn_event.spawn_data.entity);

        let spawn_pawn_data = spawn_event.builder.get_spawn_pawn_data();

        if spawn_event.spawn_data.showcase_data_option.is_none() {
            let pawn_component = spawn_event
                .builder
                .get_spawn_pawn_data()
                .pawn_component
                .clone();

            spawner.insert((
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

            match spawn_pawn_data.designation {
                PawnDesignation::Player => {
                    spawner.insert((
                        spawn_pawn_data.connected_player_option.unwrap(),
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

        let mut first_damage_flags = HashMap::new();
        first_damage_flags.insert(0, DamageFlag::SoftDamage);
        spawner.insert((
            Humanoid::default(),
            WorldMode {
                mode: WorldModes::Kinematic,
            },
            MeleeCombat {
                combat_melee_damage_model: DamageModel {
                    brute: 5.,
                    damage_flags: first_damage_flags,
                    ..Default::default()
                },
                ..Default::default()
            },
        ));

        spawner
            .insert(Dominance::group(10))
            .insert(LockedAxes::ROTATION_LOCKED);

        let mut slot_entities: HashMap<String, Entity> = HashMap::new();

        for (slot_name, item_name) in spawn_pawn_data.inventory_setup.iter() {
            let entity_option;

            entity_option = spawn_held_entity(
                item_name.clone(),
                &mut commands,
                spawn_event.spawn_data.entity,
                spawn_event.spawn_data.showcase_data_option.clone(),
                &entity_data,
                &mut default_spawner,
            );

            match entity_option {
                Some(entity) => {
                    slot_entities.insert(slot_name.to_string(), entity);
                }
                None => {}
            }
        }

        let mut spawner = commands.entity(spawn_event.spawn_data.entity);

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
    }
}

/// Manage spawning human dummy.
#[cfg(any(feature = "server", feature = "client"))]
pub(crate) fn default_build_human_dummies(
    mut default_spawner: EventReader<DefaultSpawnEvent>,
    mut spawner: EventWriter<SpawnEntity<HumanMaleType>>,
) {
    use helmet_security::spawn::HelmetType;
    use jumpsuit_security::spawn::JumpsuitType;

    for spawn_event in default_spawner.iter() {
        if spawn_event.spawn_data.entity_type.to_string() == HUMAN_DUMMY_ENTITY_NAME {
            spawner.send(SpawnEntity {
                spawn_data: spawn_event.spawn_data.clone(),
                builder: HumanMaleType {
                    spawn_pawn_data: SpawnPawnData {
                        pawn_component: Pawn {
                            character_name: "dummy".to_string(),
                            ..Default::default()
                        },
                        inventory_setup: vec![
                            ("jumpsuit".to_string(), Box::new(JumpsuitType::default())),
                            ("helmet".to_string(), Box::new(HelmetType::default())),
                        ],
                        designation: PawnDesignation::Dummy,
                        connected_player_option: None,
                    },
                    identifier: HumanMaleType::default().to_string(),
                },
            });
        }
    }
}
