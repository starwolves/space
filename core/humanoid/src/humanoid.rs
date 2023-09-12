use std::collections::HashMap;

use bevy::{
    prelude::{warn, Component, Entity, Query, Res, ResMut, Vec3, With},
    time::{Timer, TimerMode},
};
use bevy_xpbd_3d::prelude::{ExternalForce, LinearVelocity};
use controller::controller::ControllerInput;
use entity::health::DamageFlag;
use physics::entity::{RigidBodies, SFRigidBody};
use player::connections::ServerEventBuffer;

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

pub const MOVEMENT_SPEED: f32 = 80.;
pub const MAX_MOVEMENT_SPEED: f32 = 5.;

pub(crate) fn humanoid_movement(
    humanoids: Query<(Entity, &Humanoid, &ControllerInput)>,
    rigidbodies: Res<RigidBodies>,
    mut rigidbodies_query: Query<(&mut ExternalForce, &LinearVelocity), With<SFRigidBody>>,
) {
    for (entity, _humanoid, input) in humanoids.iter() {
        let rigidbody_entity;
        match rigidbodies.get_entity_rigidbody(&entity) {
            Some(bdy) => {
                rigidbody_entity = bdy;
            }
            None => {
                warn!("Humanoid had no rigidbody.");
                continue;
            }
        }

        match rigidbodies_query.get_mut(*rigidbody_entity) {
            Ok((mut external_force, velocity)) => {
                let mut corrected_movement_vector = input.movement_vector.clone();
                if input.movement_vector.x == 1. && velocity.x > MAX_MOVEMENT_SPEED {
                    corrected_movement_vector.x = 0.;
                }
                if input.movement_vector.x == -1. && velocity.x < -MAX_MOVEMENT_SPEED {
                    corrected_movement_vector.x = 0.;
                }
                if input.movement_vector.y == 1. && velocity.z > MAX_MOVEMENT_SPEED {
                    corrected_movement_vector.y = 0.;
                }
                if input.movement_vector.y == -1. && velocity.z < -MAX_MOVEMENT_SPEED {
                    corrected_movement_vector.y = 0.;
                }

                let normalized_movement_vector = corrected_movement_vector.normalize_or_zero();

                external_force.set_force(Vec3::new(
                    normalized_movement_vector.x * MOVEMENT_SPEED,
                    0.,
                    normalized_movement_vector.y * MOVEMENT_SPEED,
                ));
            }
            Err(_rr) => {
                warn!("Couldnt find ExternalForce component");
            }
        }
    }
}

/// On player disconnect as a function.

#[allow(unused_variables)]
pub fn on_player_disconnect(
    mut humanoids: Query<&mut Humanoid>,
    handle_to_entity: ResMut<HandleToEntity>,
    reader: Res<ServerEventBuffer>,
) {
    for e in reader.buffer.iter() {
        let event = e.renet_event();
        match event {
            ServerEvent::ClientDisconnected { client_id, reason } => {
                match handle_to_entity.map.get(&client_id) {
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
