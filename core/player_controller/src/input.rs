use bevy::prelude::{warn, Entity, EventReader, Query};
use humanoid::humanoid::Humanoid;
use math::grid::Vec3Int;
use networking::server::{
    InputAltItemAttack, InputAttackEntity, InputMouseAction, InputMovementInput,
    InputSelectBodyPart, InputSprinting, InputToggleAutoMove,
};
use pawn::pawn::ControllerInput;

/// Manage controller input for humanoid. The controller can be controlled by a player or AI.
#[cfg(feature = "server")]
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

/// Manage player input and apply to controller.
#[cfg(feature = "server")]
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
#[cfg(feature = "server")]
pub struct InputAttackCell {
    pub entity: Entity,
    pub id: Vec3Int,
}
