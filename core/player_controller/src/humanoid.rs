use std::{collections::HashMap, f32::consts::PI};

use api::{
    data::{
        ConnectedPlayer, NoData, Showcase, HUMAN_DUMMY_ENTITY_NAME, HUMAN_MALE_ENTITY_NAME,
        JUMPSUIT_SECURITY_ENTITY_NAME,
    },
    data_link::{DataLink, DataLinkType},
    entity_updates::{get_entity_update_difference, EntityUpdateData, EntityUpdates},
    humanoid::UsedNames,
    inventory::{Inventory, Slot, SlotType},
    pawn::PawnDesignation,
};
use bevy::{
    math::Vec2,
    prelude::{Changed, Entity, Query},
};
use bevy::{
    math::Vec3,
    prelude::{EventReader, Transform},
};
use entity::{
    entity_data::NetShowcase,
    meta::EntityDataResource,
    spawn::{
        base_entity_builder, BaseEntityBundle, BaseEntityData, BaseEntitySummonable,
        DefaultSpawnEvent, SpawnData, SpawnEvent,
    },
};
use examinable::examine::{Examinable, RichName};
use health::core::{DamageFlag, Health, HealthContainer, HumanoidHealth};
use networking::messages::ReliableServerMessage;
use pawn::pawn::{FacingDirection, ShipAuthorization, ShipAuthorizationEnum, ShipJobsEnum};
use senser::senser::Senser;

use crate::connection::SpawnPawnData;

use std::collections::BTreeMap;

use bevy::prelude::{Commands, EventWriter, ResMut};
use inventory_item::{
    combat::{CombatAttackAnimation, DamageModel, MeleeCombat, ProjectileCombat},
    item::CombatStandardAnimation,
    spawn::spawn_held_entity,
};

use bevy_rapier3d::prelude::{CoefficientCombineRule, Collider, Dominance, Friction, LockedAxes};
use chat::chat::{Radio, RadioChannel};
use entity::entity_data::{ENTITY_SPAWN_PARENT, HELMET_SECURITY_ENTITY_NAME};
use humanoid::{
    humanoid::{CharacterAnimationState, Humanoid},
    user_name::get_dummy_name,
};
use inventory_item::item::InventoryItem;
use map::map::Map;
use pawn::pawn::{ControllerInput, Pawn, PersistentPlayerData};
use physics::{
    physics::CHARACTER_FLOOR_FRICTION,
    world_mode::{WorldMode, WorldModes},
};
use rigid_body::spawn::{RigidBodyBundle, RigidBodySummonable};

use vector2math::{FloatingVector2, Vector2};

/// All the core humanoid entity updates for the Godot client.
pub(crate) fn humanoid_core_entity_updates(
    mut updated_humans: Query<
        (
            Entity,
            Option<&Pawn>,
            &Humanoid,
            &mut EntityUpdates,
            &PersistentPlayerData,
            &Inventory,
            Option<&ControllerInput>,
            Option<&ConnectedPlayer>,
            Option<&Showcase>,
        ),
        Changed<Humanoid>,
    >,
    inventory_items: Query<(&InventoryItem, &MeleeCombat, Option<&ProjectileCombat>)>,
) {
    for (
        _entity,
        pawn_component_option,
        humanoid_component,
        mut entity_updates_component,
        persistent_player_data_component,
        inventory_component,
        player_input_option,
        connected_player_component_option,
        showcase_component_option,
    ) in updated_humans.iter_mut()
    {
        let old_entity_updates = entity_updates_component.updates.clone();

        let lower_body_animation_state: String;

        let mut upper_body_animation_state: String;

        let mut animation_tree1_upper_blend = HashMap::new();
        let mut animation_tree1_lower_body_strafe_blend_position = HashMap::new();
        let mut animation_tree1_upper_body_strafe_blend_position = HashMap::new();

        let mut upper_body_left_punch_time_scale = HashMap::new();
        let mut upper_body_right_punch_time_scale = HashMap::new();

        upper_body_left_punch_time_scale
            .insert("time_scale".to_string(), EntityUpdateData::Float(2.));
        upper_body_right_punch_time_scale
            .insert("time_scale".to_string(), EntityUpdateData::Float(2.));

        entity_updates_component.updates.insert(
            "Smoothing/pawn/humanMale/rig/animationTree1>>parameters/upperBodyState/Punching Left/TimeScale/scale".to_string(),
            upper_body_left_punch_time_scale
        );
        entity_updates_component.updates.insert(
            "Smoothing/pawn/humanMale/rig/animationTree1>>parameters/upperBodyState/Punching Right/TimeScale/scale".to_string(),
            upper_body_right_punch_time_scale
        );

        let mut active_item_entity = None;

        let is_left_handed;

        if inventory_component.active_slot == "left_hand" {
            is_left_handed = true;
        } else {
            is_left_handed = false;
        }

        let mut inventory_item_components_option = None;

        for slot in inventory_component.slots.iter() {
            if slot.slot_name == inventory_component.active_slot {
                active_item_entity = slot.slot_item;
                match active_item_entity {
                    Some(ent) => {
                        inventory_item_components_option = Some(inventory_items.get(ent).unwrap());
                    }
                    None => {}
                }

                break;
            }
        }

        let mut lower_body_strafe_blendspace2d_path = "Smoothing/pawn/humanMale/rig/animationTree1>>parameters/upperBodyState/JoggingStrafe/BlendSpace2D/blend_position".to_string();
        let upper_body_strafe_blendspace2d_path = "Smoothing/pawn/humanMale/rig/animationTree1>>parameters/mainBodyState/JoggingStrafe/BlendSpace2D/blend_position".to_string();

        let mut update_upper_body = true;

        let mut alt_attack_mode = false;

        if humanoid_component.combat_mode {
            // Here we can set upper body animations to certain combat state, eg boxing stance, melee weapon stances, projectile weapon stances etc.

            let mut upper_body_blend_amount = 1.;

            let projectile_combat_component_option;

            match humanoid_component.current_lower_animation_state {
                CharacterAnimationState::Idle => match inventory_item_components_option {
                    Some((
                        inventory_item_component,
                        _melee_combat_component,
                        projectile_combat_option,
                    )) => {
                        projectile_combat_component_option = projectile_combat_option;

                        match player_input_option {
                            Some(player_input_component) => {
                                if player_input_component.alt_attack_mode
                                    && projectile_combat_component_option.is_some()
                                {
                                    alt_attack_mode = true;
                                }
                            }
                            None => {}
                        }

                        match inventory_item_component.combat_standard_animation {
                            CombatStandardAnimation::StandardStance => {
                                upper_body_animation_state = "Idle Heightened".to_string();
                                lower_body_animation_state = "Idle".to_string();
                            }
                            CombatStandardAnimation::PistolStance => {
                                if !alt_attack_mode {
                                    upper_body_animation_state = "Pistol Idle".to_string();
                                    lower_body_animation_state = "Pistol Idle".to_string();
                                } else {
                                    upper_body_animation_state = "Idle Heightened".to_string();
                                    lower_body_animation_state = "Idle".to_string();
                                }
                            }
                        }
                    }
                    None => {
                        upper_body_animation_state = "Idle Heightened".to_string();
                        lower_body_animation_state = "Idle".to_string();
                    }
                },
                CharacterAnimationState::Jogging => {
                    // Get active item in hand and check its AnimationEnum type. If StandardMelee its JoggingStrafe, if Pistol it's PistolStrafe.

                    match inventory_item_components_option {
                        Some((
                            inventory_item_component,
                            _melee_combat_component,
                            projectile_combat_component_option,
                        )) => {
                            match player_input_option {
                                Some(player_input_component) => {
                                    if player_input_component.alt_attack_mode
                                        && projectile_combat_component_option.is_some()
                                    {
                                        alt_attack_mode = true;
                                    }
                                }
                                None => {}
                            }

                            match inventory_item_component.combat_standard_animation {
                                CombatStandardAnimation::StandardStance => {
                                    upper_body_animation_state = "JoggingStrafe".to_string();
                                    lower_body_animation_state = "JoggingStrafe".to_string();
                                }
                                CombatStandardAnimation::PistolStance => {
                                    if !alt_attack_mode {
                                        upper_body_animation_state = "PistolStrafe".to_string();
                                        lower_body_animation_state = "PistolStrafe".to_string();
                                        //upper_body_strafe_blendspace2d_path = "Smoothing/pawn/humanMale/rig/animationTree1>>parameters/upperBodyState/PistolStrafe/BlendSpace2D/blend_position".to_string();
                                        lower_body_strafe_blendspace2d_path = "Smoothing/pawn/humanMale/rig/animationTree1>>parameters/mainBodyState/PistolStrafe/BlendSpace2D/blend_position".to_string();
                                        update_upper_body = false;
                                    } else {
                                        upper_body_animation_state = "JoggingStrafe".to_string();
                                        lower_body_animation_state = "JoggingStrafe".to_string();
                                    }
                                }
                            }
                        }
                        None => {
                            upper_body_animation_state = "JoggingStrafe".to_string();
                            lower_body_animation_state = "JoggingStrafe".to_string();
                        }
                    }

                    let mut strafe_blend_position;

                    match pawn_component_option.as_ref().unwrap().facing_direction {
                        FacingDirection::UpLeft => {
                            strafe_blend_position = [-1., 1.];
                        }
                        FacingDirection::Up => {
                            strafe_blend_position = [0., 1.];
                        }
                        FacingDirection::UpRight => {
                            strafe_blend_position = [1., 1.];
                        }
                        FacingDirection::Right => {
                            strafe_blend_position = [1., 0.];
                        }
                        FacingDirection::DownRight => {
                            strafe_blend_position = [-1., -1.];
                        }
                        FacingDirection::Down => {
                            strafe_blend_position = [0., -1.];
                        }
                        FacingDirection::DownLeft => {
                            strafe_blend_position = [1., -1.];
                        }
                        FacingDirection::Left => {
                            strafe_blend_position = [-1., 0.];
                        }
                    }

                    // Further rotate this Vec2 with mouse_direction.
                    if humanoid_component.facing_direction > 0.75 * PI {
                        strafe_blend_position = strafe_blend_position.rotate(-0.5 * PI);
                    } else if humanoid_component.facing_direction > 0.5 * PI {
                        strafe_blend_position = strafe_blend_position.rotate(-0.75 * PI);
                    } else if humanoid_component.facing_direction > 0.25 * PI {
                        strafe_blend_position = strafe_blend_position.rotate(1. * PI);
                    } else if humanoid_component.facing_direction > 0. {
                        strafe_blend_position = strafe_blend_position.rotate(0.75 * PI);
                    } else if humanoid_component.facing_direction > -0.25 * PI {
                        strafe_blend_position = strafe_blend_position.rotate(0.5 * PI);
                    } else if humanoid_component.facing_direction > -0.5 * PI {
                        strafe_blend_position = strafe_blend_position.rotate(0.25 * PI);
                    } else if humanoid_component.facing_direction > -0.75 * PI {
                        strafe_blend_position = strafe_blend_position.rotate(0.);
                    } else if humanoid_component.facing_direction > -1. * PI {
                        strafe_blend_position = strafe_blend_position.rotate(-0.25 * PI);
                    }

                    animation_tree1_lower_body_strafe_blend_position.insert(
                        "blend_position".to_string(),
                        EntityUpdateData::Vec2(Vec2::new(
                            strafe_blend_position.x(),
                            strafe_blend_position.y(),
                        )),
                    );

                    animation_tree1_upper_body_strafe_blend_position.insert(
                        "blend_position".to_string(),
                        EntityUpdateData::Vec2(Vec2::new(
                            strafe_blend_position.x(),
                            strafe_blend_position.y(),
                        )),
                    );
                }
                CharacterAnimationState::Sprinting => {
                    upper_body_animation_state = "Idle Heightened".to_string();
                    lower_body_animation_state = "Sprinting".to_string();
                    upper_body_blend_amount = 0.;
                }
            }

            if humanoid_component.is_attacking {
                match active_item_entity {
                    Some(_entity) => match inventory_item_components_option
                        .unwrap()
                        .1
                        .combat_attack_animation
                    {
                        CombatAttackAnimation::OneHandedMeleePunch => {
                            if is_left_handed {
                                upper_body_animation_state = "Punching Left".to_string();
                            } else {
                                upper_body_animation_state = "Punching Right".to_string();
                            }
                        }
                        CombatAttackAnimation::PistolShot => {
                            if alt_attack_mode {
                                if is_left_handed {
                                    upper_body_animation_state = "Punching Left".to_string();
                                } else {
                                    upper_body_animation_state = "Punching Right".to_string();
                                }
                            }
                        }
                    },
                    None => {
                        if is_left_handed {
                            upper_body_animation_state = "Punching Left".to_string();
                        } else {
                            upper_body_animation_state = "Punching Right".to_string();
                        }
                    }
                }
            }
            animation_tree1_upper_blend.insert(
                "blend_amount".to_string(),
                EntityUpdateData::Float(upper_body_blend_amount),
            );
        } else {
            match humanoid_component.current_lower_animation_state {
                CharacterAnimationState::Idle => {
                    lower_body_animation_state = "Idle".to_string();
                    upper_body_animation_state = "Idle Heightened".to_string();
                }
                CharacterAnimationState::Jogging => {
                    lower_body_animation_state = "Jogging".to_string();
                    upper_body_animation_state = "Jogging".to_string();
                }
                CharacterAnimationState::Sprinting => {
                    lower_body_animation_state = "Sprinting".to_string();
                    upper_body_animation_state = "Idle Heightened".to_string();
                }
            }

            animation_tree1_upper_blend
                .insert("blend_amount".to_string(), EntityUpdateData::Float(0.));
        }

        let mut animation_tree1_upper_body_updates = HashMap::new();
        let mut animation_tree1_lower_body_updates = HashMap::new();

        animation_tree1_upper_body_updates.insert(
            "travel".to_string(),
            EntityUpdateData::String(upper_body_animation_state),
        );
        animation_tree1_lower_body_updates.insert(
            "travel".to_string(),
            EntityUpdateData::String(lower_body_animation_state),
        );

        entity_updates_component.updates.insert(
            "Smoothing/pawn/humanMale/rig/animationTree1>>parameters/upperBodyState/playback/travel".to_string(),
            animation_tree1_upper_body_updates
        );

        entity_updates_component.updates.insert(
            "Smoothing/pawn/humanMale/rig/animationTree1>>parameters/mainBodyState/playback/travel"
                .to_string(),
            animation_tree1_lower_body_updates,
        );

        entity_updates_component.updates.insert(
            "Smoothing/pawn/humanMale/rig/animationTree1>>parameters/upperBodyBlend/blend_amount"
                .to_string(),
            animation_tree1_upper_blend,
        );

        entity_updates_component.updates.insert(
            lower_body_strafe_blendspace2d_path.clone(),
            animation_tree1_lower_body_strafe_blend_position,
        );

        if update_upper_body {
            entity_updates_component.updates.insert(
                upper_body_strafe_blendspace2d_path,
                animation_tree1_upper_body_strafe_blend_position,
            );
        }

        match showcase_component_option {
            Some(_showcase_component) => {}
            None => {
                let mut billboard_username_updates = HashMap::new();

                billboard_username_updates.insert(
                    "bbcode".to_string(),
                    EntityUpdateData::String(
                        "[color=white][center][b]".to_owned()
                            + &persistent_player_data_component.character_name
                            + "[/b][/center][/color]",
                    ),
                );

                match connected_player_component_option {
                    Some(connected_player_component) => {
                        entity_updates_component.excluded_handles.insert("Smoothing/pawn/humanMale/textViewPortChat0/ViewPort/chatText/VControl/name".to_string(), vec![connected_player_component.handle]);
                    }
                    None => {}
                }

                entity_updates_component.updates.insert(
                    "Smoothing/pawn/humanMale/textViewPortChat0/ViewPort/chatText/VControl/name"
                        .to_string(),
                    billboard_username_updates,
                );
            }
        }

        let difference_updates =
            get_entity_update_difference(old_entity_updates, &entity_updates_component.updates);

        entity_updates_component
            .updates_difference
            .push(difference_updates);
    }
}

/// Get default transform.
pub fn get_default_transform() -> Transform {
    Transform::identity()
}

/// Human male spawn data.
pub struct HumanMaleSummonData {
    pub used_names: UsedNames,
}

impl BaseEntitySummonable<HumanMaleSummonData> for HumanMaleSummoner {
    fn get_bundle(
        &self,
        _spawn_data: &SpawnData,
        mut entity_data: HumanMaleSummonData,
    ) -> BaseEntityBundle {
        let character_name;

        match self.spawn_pawn_data.designation {
            PawnDesignation::Dummy => {
                character_name = get_dummy_name(&mut entity_data.used_names);
            }
            PawnDesignation::Ai => {
                character_name = "Ai".to_string();
            }
            _ => {
                character_name = self
                    .spawn_pawn_data
                    .persistent_player_data
                    .character_name
                    .clone();
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
            entity_name: HUMAN_MALE_ENTITY_NAME.to_string(),
            health: Health {
                health_container: HealthContainer::Humanoid(HumanoidHealth::default()),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

/// Human male spawner.
pub fn summon_base_human_male<
    T: BaseEntitySummonable<HumanMaleSummonData> + Send + Sync + 'static,
>(
    mut spawn_events: EventReader<SpawnEvent<T>>,
    mut commands: Commands,
    used_names: ResMut<UsedNames>,
    mut net_showcase: EventWriter<NetShowcase>,
) {
    for spawn_event in spawn_events.iter() {
        let base_entity_bundle = spawn_event.summoner.get_bundle(
            &spawn_event.spawn_data,
            HumanMaleSummonData {
                used_names: used_names.clone(),
            },
        );

        base_entity_builder(
            &mut commands,
            BaseEntityData {
                entity_type: base_entity_bundle.entity_name.clone(),
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
                net_showcase.send(NetShowcase {
                    handle: showcase_data.handle,
                    message: ReliableServerMessage::LoadEntity(
                        "entity".to_string(),
                        base_entity_bundle.entity_name,
                        HashMap::new(),
                        spawn_event.spawn_data.entity.to_bits(),
                        true,
                        "main".to_string(),
                        ENTITY_SPAWN_PARENT.to_string(),
                        false,
                    ),
                });
            }
            None => {}
        }
    }
}

/// Human male spawner.
pub struct HumanMaleSummoner {
    pub character_name: String,
    pub user_name: String,
    pub spawn_pawn_data: SpawnPawnData,
}
pub const R: f32 = 0.5;

impl RigidBodySummonable<NoData> for HumanMaleSummoner {
    fn get_bundle(&self, _spawn_data: &SpawnData, _entity_data: NoData) -> RigidBodyBundle {
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

impl HumanMaleSummonable for HumanMaleSummoner {
    fn get_character_name(&self) -> String {
        self.character_name.clone()
    }
    fn get_user_name(&self) -> String {
        self.user_name.clone()
    }
    fn get_spawn_pawn_data(&self) -> SpawnPawnData {
        self.spawn_pawn_data.clone()
    }
}

pub trait HumanMaleSummonable {
    fn get_character_name(&self) -> String;
    fn get_user_name(&self) -> String;
    fn get_spawn_pawn_data(&self) -> SpawnPawnData;
}
/// human-male specific spawn components and bundles.
pub fn summon_human_male<T: HumanMaleSummonable + Send + Sync + 'static>(
    mut commands: Commands,
    mut spawn_events: EventReader<SpawnEvent<T>>,
    mut default_spawner: EventWriter<DefaultSpawnEvent>,
    entity_data: ResMut<EntityDataResource>,
) {
    for spawn_event in spawn_events.iter() {
        let mut spawner = commands.entity(spawn_event.spawn_data.entity);

        let spawn_pawn_data = spawn_event.summoner.get_spawn_pawn_data();

        if spawn_event.spawn_data.showcase_data_option.is_none() {
            let pawn_component = Pawn {
                name: spawn_event.summoner.get_character_name().clone(),
                job: ShipJobsEnum::Security,
                ..Default::default()
            };

            spawner.remove::<Transform>();
            let mut new_transform = spawn_event.spawn_data.entity_transform;
            new_transform.translation.y = 0.9 - R;
            spawner.insert(new_transform);

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

            match spawn_pawn_data.designation {
                PawnDesignation::Player => {
                    spawner.insert_bundle((
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
        spawner.insert_bundle((
            Humanoid {
                character_name: spawn_event.summoner.get_character_name().clone(),
                ..Default::default()
            },
            PersistentPlayerData {
                character_name: spawn_event.summoner.get_character_name().clone(),
                user_name: spawn_event.summoner.get_user_name().clone(),
                ..Default::default()
            },
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
                item_name.to_string(),
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
pub(crate) fn default_human_dummy(
    mut default_spawner: EventReader<DefaultSpawnEvent>,
    mut spawner: EventWriter<SpawnEvent<HumanMaleSummoner>>,
    mut used_names: ResMut<UsedNames>,
) {
    for spawn_event in default_spawner.iter() {
        if spawn_event.spawn_data.entity_name == HUMAN_DUMMY_ENTITY_NAME {
            spawner.send(SpawnEvent {
                spawn_data: spawn_event.spawn_data.clone(),
                summoner: HumanMaleSummoner {
                    character_name: get_dummy_name(&mut used_names),
                    user_name: "DUMMY_USER_NAME".to_string(),
                    spawn_pawn_data: SpawnPawnData {
                        persistent_player_data: PersistentPlayerData::default(),
                        connected_player_option: None,
                        inventory_setup: vec![
                            (
                                "jumpsuit".to_string(),
                                JUMPSUIT_SECURITY_ENTITY_NAME.to_string(),
                            ),
                            (
                                "helmet".to_string(),
                                HELMET_SECURITY_ENTITY_NAME.to_string(),
                            ),
                        ],
                        designation: PawnDesignation::Dummy,
                    },
                },
            });
        }
    }
}
