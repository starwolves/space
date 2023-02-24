use crate::controller::ControllerInput;
use bevy::prelude::{warn, Entity, EventReader, Query, Vec2};
use resources::math::Vec3Int;

/// Manage player input and apply to controller.

pub(crate) fn apply_movement_input_controller(
    mut movement_input_event: EventReader<InputMovementInput>,
    mut sprinting_input_event: EventReader<InputSprinting>,
    mut query: Query<&mut ControllerInput>,
) {
    for new_event in movement_input_event.iter() {
        let player_entity = new_event.player_entity;

        let player_input_component_result = query.get_mut(player_entity);

        match player_input_component_result {
            Ok(mut player_input_component) => {
                player_input_component.movement_vector = new_event.vector;
            }
            Err(_rr) => {
                warn!("Couldn't process player input (movement_input_event): couldn't find player_entity.");
            }
        }
    }

    for new_event in sprinting_input_event.iter() {
        let player_entity = new_event.entity;

        let player_input_component_result = query.get_mut(player_entity);

        match player_input_component_result {
            Ok(mut player_input_component) => {
                player_input_component.sprinting = new_event.is_sprinting;
            }
            Err(_rr) => {
                warn!("Couldn't process player input (sprinting_input_event): couldn't find player_entity.");
            }
        }
    }
}

/// Client attack cell input event.

pub struct InputAttackCell {
    pub entity: Entity,
    pub id: Vec3Int,
}

/// Client input toggle combat mode event.

pub struct InputToggleCombatMode {
    pub entity: Entity,
}

/// Client input toggle auto move event.

pub struct InputToggleAutoMove {
    pub entity: Entity,
}

/// Client input attack entity event.

pub struct InputAttackEntity {
    pub entity: Entity,
    pub target_entity_bits: u64,
}

/// Client input alt item attack event.

pub struct InputAltItemAttack {
    pub entity: Entity,
}

/// Client input mouse action event.

pub struct InputMouseAction {
    pub entity: Entity,
    pub pressed: bool,
}
/// Client input select body part event.

pub struct InputSelectBodyPart {
    pub entity: Entity,
    pub body_part: String,
}

/// Client input movement event.

pub struct InputMovementInput {
    pub player_entity: Entity,
    pub vector: Vec2,
}

/// Client input sprinting event.

pub struct InputSprinting {
    pub entity: Entity,
    pub is_sprinting: bool,
}

/// Client input build graphics event.

pub struct InputBuildGraphics {
    pub handle: u64,
}

/// Client input mouse direction update event.

pub struct InputMouseDirectionUpdate {
    pub entity: Entity,
    pub direction: f32,
    pub time_stamp: u64,
}
