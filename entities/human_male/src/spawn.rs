use std::collections::{BTreeMap, HashMap};

use bevy::prelude::{warn, Commands, EventReader, EventWriter, Transform, Vec3};
use bevy_xpbd_3d::prelude::{CoefficientCombine, Collider, Friction};
use entity::{
    entity_data::{WorldMode, WorldModes},
    entity_macros::Identity,
    entity_types::EntityType,
    examine::{Examinable, RichName},
    health::{DamageFlag, Health, HealthContainer, HumanoidHealth},
    net::LoadEntity,
    senser::Senser,
    spawn::{
        base_entity_builder, BaseEntityBuilder, BaseEntityBundle, BaseEntityData, EntityBuildData,
        NoData, SpawnEntity,
    },
};
use humanoid::humanoid::{Humanoid, HUMAN_MALE_ENTITY_NAME};
use inventory::server::{
    combat::{DamageModel, MeleeCombat},
    inventory::{AddItemToSlot, AddSlot, Inventory, Slot},
};
use map::map::Map;
use pawn::pawn::{DataLink, DataLinkType, PawnBuilder};
use pawn::pawn::{PawnDesignation, ShipAuthorization, ShipAuthorizationEnum, SpawnPawnData};
use physics::physics::CHARACTER_FLOOR_FRICTION;
use physics::spawn::{RigidBodyBuilder, RigidBodyBundle};
use resources::math::Vec2Int;

/// Get default transform.

pub fn get_default_transform() -> Transform {
    Transform::IDENTITY
}

/// Human male spawn data.

pub struct HumanMaleBuildData;

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

pub fn build_base_human_males<T: BaseEntityBuilder<HumanMaleBuildData> + 'static>(
    mut spawn_events: EventReader<SpawnEntity<T>>,
    mut commands: Commands,
    mut server: EventWriter<OutgoingReliableServerMessage<EntityServerMessage>>,
    types: Res<EntityTypes>,
) {
    for spawn_event in spawn_events.iter() {
        let base_entity_bundle = spawn_event
            .entity_type
            .get_bundle(&spawn_event.spawn_data, HumanMaleBuildData);
        let entity_type = base_entity_bundle.entity_type.get_identity();
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
                    message: EntityServerMessage::LoadEntity(LoadEntity {
                        type_id: *types.netcode_types.get(&entity_type).unwrap(),
                        entity: spawn_event.spawn_data.entity,
                        translation: spawn_event.spawn_data.entity_transform.translation,
                        scale: spawn_event.spawn_data.entity_transform.scale,
                        rotation: spawn_event.spawn_data.entity_transform.rotation,
                        holder_entity: spawn_event.spawn_data.holder_entity_option,
                    }),
                });
            }
            None => {}
        }
    }
}

/// Human male spawner.

#[derive(Clone, Identity)]
pub struct HumanMaleType {
    pub identifier: String,
    pub sub_type: String,
    pub spawn_pawn_data: SpawnPawnData,
}

impl Default for HumanMaleType {
    fn default() -> Self {
        HumanMaleType {
            identifier: HUMAN_MALE_ENTITY_NAME.to_string(),
            sub_type: HUMAN_MALE_ENTITY_NAME.to_string(),
            spawn_pawn_data: SpawnPawnData::default(),
        }
    }
}

pub const R: f32 = 0.5;

impl RigidBodyBuilder<NoData> for HumanMaleType {
    fn get_bundle(&self, _spawn_data: &EntityBuildData, _entity_data: NoData) -> RigidBodyBundle {
        let mut friction = Friction::new(CHARACTER_FLOOR_FRICTION);
        friction.combine_rule = CoefficientCombine::Min;

        RigidBodyBundle {
            collider: Collider::capsule(R, 1.8 - R),
            collider_transform: Transform::from_translation(Vec3::new(0., 0.011, -0.004)),
            collider_friction: friction,
            rigidbody_dynamic: true,
            ..Default::default()
        }
    }
}

impl PawnBuilder for HumanMaleType {
    fn get_spawn_pawn_data(&self) -> SpawnPawnData {
        self.spawn_pawn_data.clone()
    }
}

use controller::controller::ControllerInput;

/// human-male specific spawn components and bundles.

pub fn build_human_males(
    mut commands: Commands,
    mut spawn_events: EventReader<SpawnEntity<HumanMaleType>>,
) {
    for spawn_event in spawn_events.iter() {
        let mut spawner = commands.entity(spawn_event.spawn_data.entity);

        let spawn_pawn_data = spawn_event.entity_type.get_spawn_pawn_data();

        if spawn_event.spawn_data.showcase_data_option.is_none() {
            let pawn_component = spawn_event
                .entity_type
                .get_spawn_pawn_data()
                .pawn_component
                .clone();

            spawner.insert((
                Senser::default(),
                ShipAuthorization {
                    access: vec![ShipAuthorizationEnum::Security],
                },
                pawn_component,
                ControllerInput::default(),
            ));

            match spawn_pawn_data.designation {
                PawnDesignation::Player => {
                    match spawn_pawn_data.connected_player_option {
                        Some(c) => {
                            spawner.insert(c);
                        }
                        None => {}
                    }

                    spawner.insert((
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
    }
}

pub fn spawn_held_item<T: Send + Sync + Default + 'static>(
    mut commands: Commands,
    mut default_spawner: EventWriter<SpawnEntity<T>>,
    mut spawn_events: EventReader<SpawnEntity<HumanMaleType>>,
    mut add_slot_item: EventWriter<AddItemToSlot>,
    mut add_slot: EventWriter<AddSlot>,
    types: Res<EntityTypes>,
) {
    for spawn_event in spawn_events.iter() {
        let spawn_pawn_data = spawn_event.entity_type.get_spawn_pawn_data();

        let mut slot_entities = vec![];

        for item_name in spawn_pawn_data.inventory_setup.iter() {
            let return_entity = commands.spawn(()).id();
            default_spawner.send(SpawnEntity {
                spawn_data: EntityBuildData {
                    entity_transform: Transform::IDENTITY,
                    correct_transform: false,
                    holder_entity_option: Some(spawn_event.spawn_data.entity),
                    default_map_spawn: false,
                    raw_entity_option: None,
                    showcase_data_option: spawn_event.spawn_data.showcase_data_option.clone(),
                    entity: return_entity,
                    held_entity_option: Some(return_entity),
                },
                entity_type: T::default(),
            });
            slot_entities.push((return_entity, item_name.get_identity()));
        }

        let mut spawner = commands.entity(spawn_event.spawn_data.entity);

        let mut test_slot = Slot::default();
        test_slot.name = "Backpack".to_string();
        test_slot.size = Vec2Int { x: 16, y: 8 };

        add_slot.send(AddSlot {
            inventory_entity: spawn_event.spawn_data.entity,
            slot: test_slot,
        });

        spawner.insert(Inventory::default());

        for (item, identity) in slot_entities {
            let net_type;
            match types.netcode_types.get(&identity) {
                Some(t) => {
                    net_type = *t;
                }
                None => {
                    warn!("Coudlnt find entity ttype");
                    continue;
                }
            }
            add_slot_item.send(AddItemToSlot {
                slot_id: 0,
                inventory_entity: spawn_event.spawn_data.entity,
                item_entity: item,
                item_type_id: net_type,
            });
        }
    }
}
