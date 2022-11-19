use std::collections::HashMap;

use bevy::{
    math::Vec2,
    prelude::{Component, Entity},
};

/// Ship authorizations for pawns.
#[derive(PartialEq)]
#[cfg(feature = "server")]
pub enum ShipAuthorizationEnum {
    Security,
    Common,
}
/// Crew jobs for pawns.
#[derive(Copy, Clone)]
#[cfg(feature = "server")]
pub enum ShipJobsEnum {
    Security,
    Control,
}
/// The component.
#[derive(Component)]
#[cfg(feature = "server")]
pub struct Pawn {
    pub name: String,
    pub job: ShipJobsEnum,
    pub facing_direction: FacingDirection,
}

#[cfg(feature = "server")]
impl Default for Pawn {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            job: ShipJobsEnum::Security,
            facing_direction: FacingDirection::Up,
        }
    }
}

/// Ship authorization component.
#[derive(Component)]
#[cfg(feature = "server")]
pub struct ShipAuthorization {
    pub access: Vec<ShipAuthorizationEnum>,
}

/// Controller input component.
#[derive(Component)]
#[cfg(feature = "server")]
pub struct ControllerInput {
    pub movement_vector: Vec2,
    pub sprinting: bool,
    pub is_mouse_action_pressed: bool,
    pub targetted_limb: String,
    pub auto_move_enabled: bool,
    pub auto_move_direction: Vec2,
    pub combat_targetted_entity: Option<Entity>,
    pub combat_targetted_cell: Option<Vec3Int>,
    pub alt_attack_mode: bool,
    pub pending_direction: Option<FacingDirection>,
}
impl Default for ControllerInput {
    fn default() -> Self {
        Self {
            movement_vector: Vec2::ZERO,
            sprinting: false,
            is_mouse_action_pressed: false,
            targetted_limb: "torso".to_string(),
            auto_move_enabled: false,
            auto_move_direction: Vec2::ZERO,
            combat_targetted_entity: None,
            combat_targetted_cell: None,
            alt_attack_mode: false,
            pending_direction: None,
        }
    }
}

/// Persistent player data component.
#[derive(Clone, Component)]
#[cfg(feature = "server")]
pub struct PersistentPlayerData {
    pub account_name_is_set: bool,
    pub character_name: String,
    pub account_name: String,
}
#[cfg(feature = "server")]
impl Default for PersistentPlayerData {
    fn default() -> Self {
        Self {
            account_name_is_set: false,
            character_name: "".to_string(),
            account_name: "".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
#[cfg(feature = "server")]
pub enum FacingDirection {
    UpLeft,
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
}

/// Facing direction to Vec2 as a function.
#[cfg(feature = "server")]
pub fn facing_direction_to_direction(direction: &FacingDirection) -> Vec2 {
    match direction {
        FacingDirection::UpLeft => Vec2::new(-1., 1.),
        FacingDirection::Up => Vec2::new(0., 1.),
        FacingDirection::UpRight => Vec2::new(1., 1.),
        FacingDirection::Right => Vec2::new(1., 0.),
        FacingDirection::DownRight => Vec2::new(1., -1.),
        FacingDirection::Down => Vec2::new(0., -1.),
        FacingDirection::DownLeft => Vec2::new(-1., -1.),
        FacingDirection::Left => Vec2::new(-1., 0.),
    }
}
use bevy::{math::Quat, prelude::Transform};
use bevy_rapier3d::na::Quaternion;
use data_converters::converters::string_transform_to_transform;
use math::grid::Vec3Int;
use serde::Deserialize;

#[cfg(feature = "server")]
pub struct PawnYAxisRotations;

#[cfg(feature = "server")]
impl PawnYAxisRotations {
    pub fn new() -> Vec<Quaternion<f32>> {
        vec![
            //0deg
            Quaternion::new(1., 0., 0., 0.),
            //45deg
            Quaternion::new(0.9238795, 0., 0.3826834, 0.),
            //90deg
            Quaternion::new(
                std::f32::consts::FRAC_1_SQRT_2,
                0.,
                std::f32::consts::FRAC_1_SQRT_2,
                0.,
            ),
            //135deg
            Quaternion::new(0.3826834, 0., 0.9238795, 0.),
            //180deg
            Quaternion::new(0., 0., 1., 0.),
            //225deg
            Quaternion::new(-0.3826834, 0., 0.9238795, 0.),
            //270deg
            Quaternion::new(
                -std::f32::consts::FRAC_1_SQRT_2,
                0.,
                std::f32::consts::FRAC_1_SQRT_2,
                0.,
            ),
            //315deg
            Quaternion::new(-0.9238795, 0., 0.3826834, 0.),
        ]
    }
}

#[derive(Clone)]
#[cfg(feature = "server")]
pub enum PawnDesignation {
    Showcase,
    Player,
    Dummy,
    Ai,
}
/// Component that contains the spawn data of a to-be-spawned entity.
#[derive(Component)]
#[cfg(feature = "server")]
pub struct Spawning {
    pub transform: Transform,
}

/// A spawn point in which players will spawn.
#[cfg(feature = "server")]
pub struct SpawnPoint {
    pub point_type: String,
    pub transform: Transform,
}

#[cfg(feature = "server")]
impl SpawnPoint {
    pub fn new(raw: &SpawnPointRaw) -> SpawnPoint {
        let mut this_transform = string_transform_to_transform(&raw.transform);

        this_transform.translation.y = 0.05;

        this_transform.rotation = Quat::IDENTITY;

        SpawnPoint {
            point_type: raw.point_type.clone(),
            transform: this_transform,
        }
    }
}

/// Raw json.
#[derive(Deserialize)]
#[cfg(feature = "server")]
pub struct SpawnPointRaw {
    pub point_type: String,
    pub transform: String,
}
/// Resource containing all available spawn points for players.
#[derive(Default)]
#[cfg(feature = "server")]
pub struct SpawnPoints {
    pub list: Vec<SpawnPoint>,
    pub i: usize,
}
/// How far an entity can reach ie with picking up items.
#[cfg(feature = "server")]
pub const REACH_DISTANCE: f32 = 3.;

use crate::examine_events::NetPawn;
use bevy::prelude::ResMut;
use bevy::prelude::{EventReader, EventWriter, Query, Res};
use chat_api::core::escape_bb;
use networking::server::ReliableServerMessage;
use resources::core::HandleToEntity;

use bevy::prelude::warn;
use console_commands::commands::CONSOLE_ERROR_COLOR;

/// Set player connection account name that also isn't already taken.
#[cfg(feature = "server")]
pub(crate) fn account_name(
    mut input_user_name_events: EventReader<InputAccountName>,
    mut persistent_player_data_query: Query<&mut PersistentPlayerData>,
    mut used_names: ResMut<UsedNames>,
    mut net_user_name_event: EventWriter<NetPawn>,
    handle_to_entity: Res<HandleToEntity>,
) {
    for event in input_user_name_events.iter() {
        match persistent_player_data_query.get_mut(event.entity) {
            Ok(mut persistent_player_data_component) => {
                if persistent_player_data_component.account_name_is_set {
                    continue;
                }

                let handle_option;

                match handle_to_entity.inv_map.get(&event.entity) {
                    Some(x) => {
                        handle_option = Some(x);
                    }
                    None => {
                        handle_option = None;
                    }
                }

                let mut user_name = escape_bb((&event.input_name).to_string(), true, true);

                if user_name.len() > 16 {
                    user_name = user_name[..16].to_string();
                }

                if used_names.account_name.contains_key(&user_name) {
                    //Already exists.

                    match handle_option {
                        Some(handle) => {
                            net_user_name_event.send(NetPawn{
                                handle: *handle,
                                message: ReliableServerMessage::ConsoleWriteLine("[color=".to_string() + CONSOLE_ERROR_COLOR + "]The provided account name is already in-use and you were assigned a default one. To change this please change the name and restart your game.[/color]"),
                            });
                        }
                        None => {}
                    }

                    continue;
                }

                if user_name.len() < 3 {
                    match handle_option {
                        Some(handle) => {
                            net_user_name_event.send(NetPawn {
                                handle: *handle,
                                message: ReliableServerMessage::ConsoleWriteLine("[color=".to_string() + CONSOLE_ERROR_COLOR + "]The provided user_name is too short. Special characters and whitespaces are not registered.[/color]"),
                            });
                        }
                        None => {}
                    }
                    continue;
                }

                persistent_player_data_component.account_name = user_name.to_string();

                used_names.account_name.insert(user_name, event.entity);

                persistent_player_data_component.account_name_is_set = true;
            }
            Err(_rr) => {
                warn!("Couldnt find persistent_player_data_component in query.");
            }
        }
    }
}

/// Resource keeping track of which in-game character names are taken.
#[derive(Default, Clone)]
#[cfg(feature = "server")]
pub struct UsedNames {
    /// Character names.
    pub names: HashMap<String, Entity>,
    /// Global user names.
    pub account_name: HashMap<String, Entity>,
    pub player_i: u32,
    pub dummy_i: u32,
}

/// Client input user name event.
#[cfg(feature = "server")]
pub struct InputAccountName {
    pub entity: Entity,
    pub input_name: String,
}
