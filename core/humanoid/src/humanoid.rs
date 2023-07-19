use std::collections::HashMap;

use bevy::{
    prelude::{warn, Component, Entity, EventReader, Query, ResMut},
    time::{Timer, TimerMode},
};
use controller::controller::ControllerInput;
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
            is_attacking: false,
            next_attack_timer: t,
        }
    }
}

const _JOG_SPEED: f32 = 3031.44;

const _RUN_SPEED: f32 = 3031.44;

const _MAX_JOG_SPEED: f32 = 10.;

const _MAX_RUN_SPEED: f32 = 14.;

const _COMBAT_ROTATION_SPEED: f32 = 18.;

const _DOWN_FORCE: f32 = -1.0;

use networking::server::HandleToEntity;

use const_format::concatcp;
use resources::content::SF_CONTENT_PREFIX;

pub const HUMAN_DUMMY_ENTITY_NAME: &str = concatcp!(SF_CONTENT_PREFIX, "human_dummy");

pub const HUMAN_MALE_ENTITY_NAME: &str = concatcp!(SF_CONTENT_PREFIX, "human_male");

use bevy_renet::renet::ServerEvent;

pub(crate) fn humanoid_movement(humanoids: Query<(&Humanoid, &ControllerInput)>) {}

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
