use crate::{
    controller::ControllerInput,
    net::{ControllerClientMessage, MovementInput},
};
use bevy::prelude::{
    warn, Entity, Event, EventReader, EventWriter, KeyCode, Query, Res, ResMut, Vec2,
};
use entity::spawn::PawnId;
use networking::client::OutgoingReliableClientMessage;
use resources::{
    input::{
        InputBuffer, KeyBind, KeyBinds, KeyCodeEnum, HOLD_SPRINT_BIND, JUMP_BIND,
        MOVE_BACKWARD_BIND, MOVE_FORWARD_BIND, MOVE_LEFT_BIND, MOVE_RIGHT_BIND,
    },
    math::Vec3Int,
};
/// Client attack cell input event.
#[derive(Event)]
pub struct InputAttackCell {
    pub entity: Entity,
    pub id: Vec3Int,
}

/// Client input toggle combat mode event.
#[derive(Event)]
pub struct InputToggleCombatMode {
    pub entity: Entity,
}

/// Client input toggle auto move event.
#[derive(Event)]
pub struct InputToggleAutoMove {
    pub entity: Entity,
}

/// Client input attack entity event.
#[derive(Event)]
pub struct InputAttackEntity {
    pub entity: Entity,
    pub target_entity_bits: u64,
}

/// Client input alt item attack event.
#[derive(Event)]
pub struct InputAltItemAttack {
    pub entity: Entity,
}

/// Client input mouse action event.
#[derive(Event)]
pub struct InputMouseAction {
    pub entity: Entity,
    pub pressed: bool,
}
/// Client input movement event.
#[derive(Event)]
pub struct InputMovementInput {
    pub player_entity: Entity,
    pub up: bool,
    pub left: bool,
    pub right: bool,
    pub down: bool,
    pub pressed: bool,
}

/// Client input sprinting event.
#[derive(Event)]
pub struct InputSprinting {
    pub entity: Entity,
    pub is_sprinting: bool,
}

/// Client input build graphics event.
#[derive(Event)]
pub struct InputBuildGraphics {
    pub handle: u64,
}

/// Client input mouse direction update event.
#[derive(Event)]
pub struct InputMouseDirectionUpdate {
    pub entity: Entity,
    pub direction: f32,
    pub time_stamp: u64,
}

pub(crate) fn create_input_map(mut map: ResMut<KeyBinds>) {
    map.list.insert(
        MOVE_FORWARD_BIND.to_string(),
        KeyBind {
            key_code: KeyCodeEnum::Keyboard(KeyCode::W),
            description: "Moves the player forward.".to_string(),
            name: "Move Forward".to_string(),
            customizable: true,
        },
    );
    map.list.insert(
        MOVE_BACKWARD_BIND.to_string(),
        KeyBind {
            key_code: KeyCodeEnum::Keyboard(KeyCode::S),
            description: "Moves the player backward.".to_string(),
            name: "Move Backward".to_string(),
            customizable: true,
        },
    );
    map.list.insert(
        MOVE_LEFT_BIND.to_string(),
        KeyBind {
            key_code: KeyCodeEnum::Keyboard(KeyCode::A),
            description: "Moves the player left.".to_string(),
            name: "Move Left".to_string(),
            customizable: true,
        },
    );
    map.list.insert(
        MOVE_RIGHT_BIND.to_string(),
        KeyBind {
            key_code: KeyCodeEnum::Keyboard(KeyCode::D),
            description: "Moves the player right.".to_string(),
            name: "Move Right".to_string(),
            customizable: true,
        },
    );
    map.list.insert(
        JUMP_BIND.to_string(),
        KeyBind {
            key_code: KeyCodeEnum::Keyboard(KeyCode::Space),
            description: "Jump into the air.".to_string(),
            name: "Jump".to_string(),
            customizable: true,
        },
    );
    map.list.insert(
        HOLD_SPRINT_BIND.to_string(),
        KeyBind {
            key_code: KeyCodeEnum::Keyboard(KeyCode::ShiftLeft),
            description: "Hold to sprint.".to_string(),
            name: "Sprint".to_string(),
            customizable: true,
        },
    );
}

pub(crate) fn get_client_input(
    keyboard: Res<InputBuffer>,
    mut net: EventWriter<OutgoingReliableClientMessage<ControllerClientMessage>>,
    mut movement_event: EventWriter<InputMovementInput>,
    pawn_id: Res<PawnId>,
) {
    let pawn_entity;
    match pawn_id.client {
        Some(i) => {
            pawn_entity = i;
        }
        None => {
            return;
        }
    }
    let mut pressed_movement = MovementInput::default();
    pressed_movement.pressed = true;
    let mut released_movement = MovementInput::default();
    let mut new_pressed_input = false;
    let mut new_released_input = false;

    if keyboard.just_pressed(MOVE_FORWARD_BIND) {
        pressed_movement.up = true;
        new_pressed_input = true;
    }
    if keyboard.just_pressed(MOVE_BACKWARD_BIND) {
        pressed_movement.down = true;
        new_pressed_input = true;
    }
    if keyboard.just_pressed(MOVE_LEFT_BIND) {
        pressed_movement.left = true;
        new_pressed_input = true;
    }
    if keyboard.just_pressed(MOVE_RIGHT_BIND) {
        pressed_movement.right = true;
        new_pressed_input = true;
    }

    if keyboard.just_released(MOVE_FORWARD_BIND) {
        released_movement.up = true;
        new_released_input = true;
    }
    if keyboard.just_released(MOVE_BACKWARD_BIND) {
        released_movement.down = true;
        new_released_input = true;
    }
    if keyboard.just_released(MOVE_LEFT_BIND) {
        released_movement.left = true;
        new_released_input = true;
    }
    if keyboard.just_released(MOVE_RIGHT_BIND) {
        released_movement.right = true;
        new_released_input = true;
    }
    if new_pressed_input {
        movement_event.send(InputMovementInput {
            player_entity: pawn_entity,
            up: pressed_movement.up,
            left: pressed_movement.left,
            right: pressed_movement.right,
            down: pressed_movement.down,
            pressed: pressed_movement.pressed,
        });
        net.send(OutgoingReliableClientMessage {
            message: ControllerClientMessage::MovementInput(pressed_movement),
        });
    }
    if new_released_input {
        movement_event.send(InputMovementInput {
            player_entity: pawn_entity,
            up: released_movement.up,
            left: released_movement.left,
            right: released_movement.right,
            down: released_movement.down,
            pressed: released_movement.pressed,
        });
        net.send(OutgoingReliableClientMessage {
            message: ControllerClientMessage::MovementInput(released_movement),
        });
    }
}

/// Manage controller input for humanoid. The controller can be controlled by a player or AI.
pub(crate) fn controller_input(
    mut alternative_item_attack_events: EventReader<InputAltItemAttack>,
    mut input_attack_entity: EventReader<InputAttackEntity>,
    mut input_attack_cell: EventReader<InputAttackCell>,
    mut input_mouse_action_events: EventReader<InputMouseAction>,
    mut input_toggle_auto_move: EventReader<InputToggleAutoMove>,
    mut humanoids_query: Query<&mut ControllerInput>,
    mut toggle_combat_mode_events: EventReader<InputToggleCombatMode>,

    mut movement_input_event: EventReader<InputMovementInput>,
    mut sprinting_input_event: EventReader<InputSprinting>,
) {
    for new_event in movement_input_event.iter() {
        let player_entity = new_event.player_entity;

        let player_input_component_result = humanoids_query.get_mut(player_entity);

        match player_input_component_result {
            Ok(mut player_input_component) => {
                let mut additive = Vec2::default();

                if !new_event.pressed {
                    additive *= -1.;
                }

                player_input_component.movement_vector += additive;
            }
            Err(_rr) => {
                warn!("Couldn't process player input (movement_input_event): couldn't find player_entity 0. {:?}", player_entity);
            }
        }
    }

    for new_event in sprinting_input_event.iter() {
        let player_entity = new_event.entity;

        let player_input_component_result = humanoids_query.get_mut(player_entity);

        match player_input_component_result {
            Ok(mut player_input_component) => {
                player_input_component.sprinting = new_event.is_sprinting;
            }
            Err(_rr) => {
                warn!("Couldn't process player input (sprinting_input_event): couldn't find player_entity.");
            }
        }
    }
    for event in toggle_combat_mode_events.iter() {
        match humanoids_query.get_mut(event.entity) {
            Ok(mut controller) => {
                controller.combat_mode = !controller.combat_mode;
            }
            Err(_rr) => {}
        }
    }
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
