use std::{collections::HashMap, f32::consts::PI};

use bevy::{
    prelude::{warn, Component, Entity, EventReader, Query, ResMut},
    time::{Timer, TimerMode},
};
use entity::health::DamageFlag;

use std::time::Duration;

/// Component link repeated footstep sfx with an entity.
#[derive(Component)]

pub(crate) struct LinkedFootstepsSprinting {
    pub _entity: Entity,
}

/// Component link repeated footstep sfx with an entity.
#[derive(Component)]

pub(crate) struct LinkedFootstepsWalking {
    pub _entity: Entity,
}

/// Humanoid character animation state.

pub enum CharacterAnimationState {
    Idle,
    Jogging,
    Sprinting,
}

const FIRST_MELEE_TIME: u64 = 433;

/// The humanoid component.
#[derive(Component)]

pub struct Humanoid {
    /// Lower body blended animation state of humanoid.
    pub current_lower_animation_state: CharacterAnimationState,
    /// Whether the humanoid is in combat mode or normal mode.
    pub combat_mode: bool,
    /// The the humanoid is facing.
    pub facing_direction: f32,
    /// If attacking this frame.
    pub is_attacking: bool,
    /// Timeout between attacks.
    pub next_attack_timer: Timer,
}

impl Default for Humanoid {
    fn default() -> Self {
        let mut t = Timer::new(Duration::from_millis(FIRST_MELEE_TIME), TimerMode::Once);
        let mut first_damage_flags = HashMap::new();
        first_damage_flags.insert(0, DamageFlag::SoftDamage);
        t.tick(Duration::from_millis(FIRST_MELEE_TIME));
        Self {
            current_lower_animation_state: CharacterAnimationState::Idle,
            combat_mode: false,
            facing_direction: 0.,
            is_attacking: false,
            next_attack_timer: t,
        }
    }
}
use controller::input::InputToggleCombatMode;

/// Toggle combat mode. Ie from melee to projectile.

pub(crate) fn toggle_combat_mode(
    mut toggle_combat_mode_events: EventReader<InputToggleCombatMode>,
    mut standard_character_query: Query<&mut Humanoid>,
) {
    for event in toggle_combat_mode_events.iter() {
        match standard_character_query.get_mut(event.entity) {
            Ok(mut standard_character) => {
                standard_character.combat_mode = !standard_character.combat_mode;
            }
            Err(_rr) => {}
        }
    }
}

const _JOG_SPEED: f32 = 3031.44;

const _RUN_SPEED: f32 = 3031.44;

const _MAX_JOG_SPEED: f32 = 10.;

const _MAX_RUN_SPEED: f32 = 14.;

const _COMBAT_ROTATION_SPEED: f32 = 18.;

const _DOWN_FORCE: f32 = -1.0;

use controller::controller::ControllerInput;
use networking::server::HandleToEntity;

use const_format::concatcp;
use resources::content::SF_CONTENT_PREFIX;

pub const HUMAN_DUMMY_ENTITY_NAME: &str = concatcp!(SF_CONTENT_PREFIX, "human_dummy");

pub const HUMAN_MALE_ENTITY_NAME: &str = concatcp!(SF_CONTENT_PREFIX, "human_male");
use controller::input::InputAltItemAttack;
use controller::input::InputAttackCell;
use controller::input::InputAttackEntity;
use controller::input::InputMouseAction;
use controller::input::InputSelectBodyPart;
use controller::input::InputToggleAutoMove;

pub(crate) fn humanoid_movement() {}

/// Manage controller input for humanoid. The controller can be controlled by a player or AI.
pub(crate) fn humanoid_controller_input(
    mut alternative_item_attack_events: EventReader<InputAltItemAttack>,
    mut input_attack_entity: EventReader<InputAttackEntity>,
    mut input_attack_cell: EventReader<InputAttackCell>,
    mut input_mouse_action_events: EventReader<InputMouseAction>,
    mut input_select_body_part: EventReader<InputSelectBodyPart>,
    mut input_toggle_auto_move: EventReader<InputToggleAutoMove>,
    mut humanoids_query: Query<(&Humanoid, &mut ControllerInput)>,
) {
    for event in alternative_item_attack_events.iter() {
        match humanoids_query.get_component_mut::<ControllerInput>(event.entity) {
            Ok(mut controller_input_component) => {
                controller_input_component.alt_attack_mode =
                    !controller_input_component.alt_attack_mode;
            }
            Err(_rr) => {
                warn!("Couldn't find standard_character_component belonging to entity of InputAltItemAttack.");
            }
        }
    }

    for event in input_attack_cell.iter() {
        match humanoids_query.get_component_mut::<ControllerInput>(event.entity) {
            Ok(mut controller_input_component) => {
                controller_input_component.combat_targetted_cell = Some(event.id);
            }
            Err(_rr) => {
                warn!("Couldn't find standard_character_component belonging to entity of input_attack_cell.");
            }
        }
    }

    for event in input_attack_entity.iter() {
        match humanoids_query.get_component_mut::<ControllerInput>(event.entity) {
            Ok(mut played_input_component) => {
                played_input_component.combat_targetted_entity =
                    Some(Entity::from_bits(event.target_entity_bits));
            }
            Err(_rr) => {
                warn!("Couldn't find standard_character_component belonging to entity of InputAttackEntity.");
            }
        }
    }

    for event in input_mouse_action_events.iter() {
        match humanoids_query.get_component_mut::<ControllerInput>(event.entity) {
            Ok(mut played_input_component) => {
                played_input_component.is_mouse_action_pressed = event.pressed;

                if !event.pressed {
                    played_input_component.combat_targetted_entity = None;
                    played_input_component.combat_targetted_cell = None;
                }
            }
            Err(_rr) => {
                warn!("Couldn't find standard_character_component belonging to entity of InputMouseAction.");
            }
        }
    }

    for event in input_select_body_part.iter() {
        match humanoids_query.get_component_mut::<ControllerInput>(event.entity) {
            Ok(mut player_input_component) => {
                player_input_component.targetted_limb = event.body_part.clone();
            }
            Err(_rr) => {
                warn!("Couldnt find PlayerInput entity for input_select_body_part");
            }
        }
    }

    for event in input_toggle_auto_move.iter() {
        match humanoids_query.get_component_mut::<ControllerInput>(event.entity) {
            Ok(mut player_input_component) => {
                player_input_component.auto_move_enabled =
                    !player_input_component.auto_move_enabled;
            }
            Err(_rr) => {
                warn!("Couldnt find PlayerInput entity for input_toggle_auto_move");
            }
        }
    }
}
use bevy_renet::renet::ServerEvent;

/// On player disconnect as a function.

#[allow(unused_variables)]
pub fn on_player_disconnect(
    mut humanoids: Query<&mut Humanoid>,
    handle_to_entity: ResMut<HandleToEntity>,
    mut reader: EventReader<ServerEvent>,
) {
    for event in reader.iter() {
        match event {
            ServerEvent::ClientDisconnected { client_id, reason } => {
                match handle_to_entity.map.get(client_id) {
                    Some(ent) => match humanoids.get_mut(*ent) {
                        Ok(mut humanoid_component) => {
                            humanoid_component.current_lower_animation_state =
                                CharacterAnimationState::Idle;
                        }
                        Err(_rr) => {
                            warn!("on_player_disconnect couldnt find humanoid_component.");
                        }
                    },
                    None => {
                        warn!("on_player_disconnect couldnt find entity of handle.");
                    }
                }
            }
            _ => (),
        }
    }
}
use bevy::prelude::Local;

/// Used to calculate ping for client.
#[derive(Default)]

pub(crate) struct TimeStampPerEntity {
    pub data: HashMap<Entity, u64>,
}
use controller::input::InputMouseDirectionUpdate;

/// Manage mouse direction updates.

pub(crate) fn mouse_direction_update(
    mut update_events: EventReader<InputMouseDirectionUpdate>,
    mut standard_characters: Query<&mut Humanoid>,
    mut time_stamp_per_entity: Local<TimeStampPerEntity>,
) {
    for event in update_events.iter() {
        match time_stamp_per_entity.data.get(&event.entity) {
            Some(time_stamp) => {
                if time_stamp > &event.time_stamp {
                    continue;
                }
            }
            None => {}
        }

        time_stamp_per_entity
            .data
            .insert(event.entity, event.time_stamp);

        match standard_characters.get_mut(event.entity) {
            Ok(mut standard_character_component) => {
                if standard_character_component.combat_mode == false {
                    continue;
                }

                let direction = event.direction.clamp(-PI, PI);

                standard_character_component.facing_direction = direction;
            }
            Err(_rr) => {}
        }
    }
}
