use std::collections::HashMap;

use crate::{
    controller::ControllerInput,
    net::{ControllerClientMessage, MovementInput},
    networking::{PeerReliableControllerMessage, PeerUnreliableControllerMessage},
};
use bevy::{
    log::info,
    prelude::{
        Entity, Event, EventReader, EventWriter, KeyCode, Query, Res, ResMut, Resource, SystemSet,
        Vec2,
    },
};
use bevy::{log::warn, math::Vec3};

use bevy_renet::renet::ClientId;
use cameras::LookTransform;
use entity::spawn::{PawnId, PeerPawns};
use networking::{
    client::{
        IncomingReliableServerMessage, IncomingUnreliableServerMessage,
        OutgoingReliableClientMessage,
    },
    stamp::TickRateStamp,
};
use resources::{
    correction::StartCorrection,
    input::{
        InputBuffer, KeyBind, KeyBinds, KeyCodeEnum, HOLD_SPRINT_BIND, JUMP_BIND,
        MOVE_BACKWARD_BIND, MOVE_FORWARD_BIND, MOVE_LEFT_BIND, MOVE_RIGHT_BIND,
    },
};

/// Label for systems ordering.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum InputSet {
    First,
}

/// Client input movement event.
#[derive(Event, Debug)]
pub struct InputMovementInput {
    pub player_entity: Entity,
    pub up: bool,
    pub left: bool,
    pub right: bool,
    pub down: bool,
    pub pressed: bool,
}

impl Default for InputMovementInput {
    fn default() -> Self {
        Self {
            player_entity: Entity::from_bits(0),
            up: false,
            left: false,
            right: false,
            down: false,
            pressed: false,
        }
    }
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
#[derive(Clone)]
pub enum RecordedInput {
    Reliable(PeerReliableControllerMessage),
    Unreliable(PeerUnreliableControllerMessage),
}
#[derive(Event)]
pub struct PeerSyncLookTransform {
    pub entity: Entity,
    pub target: Vec3,
    pub handle: ClientId,
    pub stamp: u64,
}

#[derive(Event, Default, Resource)]
pub struct LastPeerLookTransform {
    pub map: HashMap<ClientId, u64>,
}

pub(crate) fn apply_peer_sync_transform(
    mut events: EventReader<PeerSyncLookTransform>,
    mut query: Query<&mut LookTransform>,
    mut last: ResMut<LastPeerLookTransform>,
) {
    for event in events.read() {
        match last.map.get_mut(&event.handle) {
            Some(old_stamp) => {
                if event.stamp > *old_stamp {
                    *old_stamp = event.stamp;
                    //info!("Peer target: {:?}:{}", event.target, event.stamp);

                    match query.get_mut(event.entity) {
                        Ok(mut l) => {
                            l.target = event.target;
                        }
                        Err(_) => {
                            warn!("Couldnt find looktransform for sync.");
                        }
                    }
                }
            }
            None => {
                last.map.insert(event.handle, event.stamp);
            }
        }
    }
}

#[derive(Resource, Default, Clone)]
pub struct RecordedControllerInput {
    pub input: HashMap<u64, Vec<RecordedInput>>,
}

pub(crate) fn get_peer_input(
    mut peer: EventReader<IncomingReliableServerMessage<PeerReliableControllerMessage>>,
    mut unreliable_peer: EventReader<
        IncomingUnreliableServerMessage<PeerUnreliableControllerMessage>,
    >,
    mut record: ResMut<RecordedControllerInput>,
    stamp: Res<TickRateStamp>,
    mut movement_input_event: EventWriter<InputMovementInput>,
    mut sync: EventWriter<PeerSyncLookTransform>,
    peer_pawns: Res<PeerPawns>,
    mut start_correction: EventWriter<StartCorrection>,
) {
    let mut new_correction = false;
    let mut earliest_tick = 0;
    for message in peer.read() {
        let large_stamp = stamp.calculate_large(message.message.client_stamp);
        let msg = RecordedInput::Reliable(message.message.clone());

        match record.input.get_mut(&large_stamp) {
            Some(v) => {
                v.push(msg);
            }
            None => {
                record.input.insert(large_stamp, vec![msg]);
            }
        }
        match &message.message.message {
            ControllerClientMessage::MovementInput(input) => {
                match peer_pawns
                    .map
                    .get(&ClientId::from_raw(message.message.peer_handle.into()))
                {
                    Some(peer) => {
                        movement_input_event.send(InputMovementInput {
                            player_entity: *peer,
                            up: input.up,
                            left: input.left,
                            right: input.right,
                            down: input.down,
                            pressed: input.pressed,
                        });
                        new_correction = true;
                        let e = stamp.calculate_large(message.message.client_stamp);
                        if e < earliest_tick || earliest_tick == 0 {
                            earliest_tick = e;
                        }
                    }
                    None => {
                        warn!("Couldnt find peer pawn.");
                    }
                }
            }
        }
    }
    for message in unreliable_peer.read() {
        let large_stamp = stamp.calculate_large(message.message.client_stamp);
        let msg = RecordedInput::Unreliable(message.message.clone());
        match record.input.get_mut(&large_stamp) {
            Some(v) => {
                v.push(msg);
            }
            None => {
                record.input.insert(large_stamp, vec![msg]);
            }
        }
        match &message.message.message {
            pawn::net::UnreliableControllerClientMessage::SyncLookTransform(target) => {
                match peer_pawns
                    .map
                    .get(&ClientId::from_raw(message.message.peer_handle.into()))
                {
                    Some(peer) => {
                        let e = stamp.calculate_large(message.message.client_stamp);
                        sync.send(PeerSyncLookTransform {
                            entity: *peer,
                            target: *target,
                            handle: ClientId::from_raw(message.message.peer_handle.into()),
                            stamp: e,
                        });
                        new_correction = true;

                        if e < earliest_tick || earliest_tick == 0 {
                            earliest_tick = e;
                        }
                    }
                    None => {
                        warn!("Couldnt find peer pawn 2.");
                    }
                }
            }
        }
    }
    // Doesnt send StartCorrect if peer input is for our exact tick or future tack.
    if new_correction && earliest_tick < stamp.large {
        start_correction.send(StartCorrection {
            start_tick: earliest_tick,
            last_tick: stamp.large,
        });
    }
}

pub(crate) fn clean_recorded_input(
    mut record: ResMut<RecordedControllerInput>,
    stamp: Res<TickRateStamp>,
) {
    let mut to_remove = vec![];
    for recorded_stamp in record.input.keys() {
        if stamp.large >= 256 && recorded_stamp < &(stamp.large - 256) {
            to_remove.push(*recorded_stamp);
        }
    }
    for i in to_remove {
        record.input.remove(&i);
    }
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
    if keyboard.just_pressed(MOVE_FORWARD_BIND) {
        info!("Sending move forward.");
        movement_event.send(InputMovementInput {
            player_entity: pawn_entity,
            up: true,
            pressed: true,
            ..Default::default()
        });
        net.send(OutgoingReliableClientMessage {
            message: ControllerClientMessage::MovementInput(MovementInput {
                up: true,
                pressed: true,
                ..Default::default()
            }),
        });
    }
    if keyboard.just_pressed(MOVE_BACKWARD_BIND) {
        info!("Sending move backward.");

        movement_event.send(InputMovementInput {
            player_entity: pawn_entity,
            down: true,
            pressed: true,
            ..Default::default()
        });
        net.send(OutgoingReliableClientMessage {
            message: ControllerClientMessage::MovementInput(MovementInput {
                down: true,
                pressed: true,
                ..Default::default()
            }),
        });
    }
    if keyboard.just_pressed(MOVE_LEFT_BIND) {
        info!("Sending move left.");

        movement_event.send(InputMovementInput {
            player_entity: pawn_entity,
            left: true,
            pressed: true,
            ..Default::default()
        });
        net.send(OutgoingReliableClientMessage {
            message: ControllerClientMessage::MovementInput(MovementInput {
                left: true,
                pressed: true,
                ..Default::default()
            }),
        });
    }
    if keyboard.just_pressed(MOVE_RIGHT_BIND) {
        info!("Sending move right.");

        movement_event.send(InputMovementInput {
            player_entity: pawn_entity,
            right: true,
            pressed: true,
            ..Default::default()
        });
        net.send(OutgoingReliableClientMessage {
            message: ControllerClientMessage::MovementInput(MovementInput {
                right: true,
                pressed: true,
                ..Default::default()
            }),
        });
    }

    if keyboard.just_released(MOVE_FORWARD_BIND) {
        info!("Sending released forward.");

        movement_event.send(InputMovementInput {
            player_entity: pawn_entity,
            up: true,
            pressed: false,
            ..Default::default()
        });
        net.send(OutgoingReliableClientMessage {
            message: ControllerClientMessage::MovementInput(MovementInput {
                up: true,
                pressed: false,
                ..Default::default()
            }),
        });
    }
    if keyboard.just_released(MOVE_BACKWARD_BIND) {
        info!("Sending released backward.");

        movement_event.send(InputMovementInput {
            player_entity: pawn_entity,
            down: true,
            pressed: false,
            ..Default::default()
        });
        net.send(OutgoingReliableClientMessage {
            message: ControllerClientMessage::MovementInput(MovementInput {
                down: true,
                pressed: false,
                ..Default::default()
            }),
        });
    }
    if keyboard.just_released(MOVE_LEFT_BIND) {
        info!("Sending released left.");

        movement_event.send(InputMovementInput {
            player_entity: pawn_entity,
            left: true,
            pressed: false,
            ..Default::default()
        });
        net.send(OutgoingReliableClientMessage {
            message: ControllerClientMessage::MovementInput(MovementInput {
                left: true,
                pressed: false,
                ..Default::default()
            }),
        });
    }
    if keyboard.just_released(MOVE_RIGHT_BIND) {
        info!("Sending released right.");
        movement_event.send(InputMovementInput {
            player_entity: pawn_entity,
            right: true,
            pressed: false,
            ..Default::default()
        });
        net.send(OutgoingReliableClientMessage {
            message: ControllerClientMessage::MovementInput(MovementInput {
                right: true,
                pressed: false,
                ..Default::default()
            }),
        });
    }
}

/// Label for systems ordering.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]

pub enum Controller {
    Input,
}

/// Manage controller input for humanoid. The controller can be controlled by a player or AI.
pub(crate) fn controller_input(
    mut humanoids_query: Query<&mut ControllerInput>,

    mut movement_input_event: EventReader<InputMovementInput>,
) {
    for new_event in movement_input_event.read() {
        let player_entity = new_event.player_entity;

        let player_input_component_result = humanoids_query.get_mut(player_entity);

        match player_input_component_result {
            Ok(mut player_input_component) => {
                let mut additive = Vec2::default();

                if new_event.left {
                    additive.x = -1.;
                } else if new_event.right {
                    additive.x = 1.;
                } else if new_event.up {
                    additive.y = -1.;
                } else if new_event.down {
                    additive.y = 1.;
                }

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
}
