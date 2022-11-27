use bevy::{math::Vec2, prelude::Component};

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
    pub communicator: Communicator,
    pub facing_direction: FacingDirection,
}

/// The kind of communicator.
#[derive(Clone)]
#[cfg(feature = "server")]
pub enum Communicator {
    Standard,
    Machine,
}

#[cfg(feature = "server")]
impl Default for Pawn {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            job: ShipJobsEnum::Security,
            facing_direction: FacingDirection::Up,
            communicator: Communicator::Standard,
        }
    }
}

/// Ship authorization component.
#[derive(Component)]
#[cfg(feature = "server")]
pub struct ShipAuthorization {
    pub access: Vec<ShipAuthorizationEnum>,
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
use bevy_rapier3d::na::Quaternion;

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

/// How far an entity can reach ie with picking up items.
#[cfg(feature = "server")]
pub const REACH_DISTANCE: f32 = 3.;

use bevy::prelude::ResMut;
use bevy::prelude::{EventReader, Query, Res};
use text_api::core::escape_bb;

use bevy::prelude::warn;
use text_api::core::CONSOLE_ERROR_COLOR;

use bevy_renet::renet::RenetServer;
use networking::server::HandleToEntity;
use player::{
    boarding::PersistentPlayerData,
    names::{InputAccountName, UsedNames},
};

/// Set player connection account name that also isn't already taken.
#[cfg(feature = "server")]
pub(crate) fn account_name(
    mut input_user_name_events: EventReader<InputAccountName>,
    mut persistent_player_data_query: Query<&mut PersistentPlayerData>,
    mut used_names: ResMut<UsedNames>,
    mut server: ResMut<RenetServer>,
    handle_to_entity: Res<HandleToEntity>,
) {
    use console_commands::networking::ConsoleCommandsServerMessage;
    use networking::plugin::RENET_RELIABLE_CHANNEL_ID;

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
                            server.send_message(*handle, RENET_RELIABLE_CHANNEL_ID, bincode::serialize(&ConsoleCommandsServerMessage::ConsoleWriteLine("[color=".to_string() + CONSOLE_ERROR_COLOR + "]The provided account name is already in-use and you were assigned a default one. To change this please change the name and restart your game.[/color]")).unwrap());
                        }
                        None => {}
                    }

                    continue;
                }

                if user_name.len() < 3 {
                    match handle_option {
                        Some(handle) => {
                            server.send_message(*handle, RENET_RELIABLE_CHANNEL_ID, bincode::serialize(&ConsoleCommandsServerMessage::ConsoleWriteLine("[color=".to_string() + CONSOLE_ERROR_COLOR + "]The provided user_name is too short. Special characters and whitespaces are not registered.[/color]")).unwrap());
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
