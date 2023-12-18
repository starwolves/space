use std::collections::HashMap;

use crate::{
    controller::ControllerInput,
    net::{ControllerClientMessage, MovementInput, PeerControllerClientMessage},
    networking::{PeerReliableControllerMessage, PeerUnreliableControllerMessage},
};
use bevy::{
    ecs::system::Local,
    input::Input,
    log::info,
    prelude::{
        Entity, Event, EventReader, EventWriter, KeyCode, Query, Res, ResMut, Resource, SystemSet,
        Vec2,
    },
    transform::components::Transform,
};
use bevy::{log::warn, math::Vec3};

use bevy_renet::renet::{ClientId, RenetClient};
use cameras::{LookTransform, LookTransformCache};
use entity::spawn::{PawnId, PeerPawns};
use itertools::Itertools;
use networking::{
    client::{IncomingReliableServerMessage, IncomingUnreliableServerMessage},
    messaging::{ReliableClientMessageBatch, ReliableMessage, Typenames},
    plugin::RENET_RELIABLE_ORDERED_ID,
    stamp::TickRateStamp,
};
use physics::{
    cache::{PhysicsCache, PriorityPhysicsCache, PriorityUpdate},
    sync::ClientStartedSyncing,
};
use resources::{
    core::TickRate,
    correction::{StartCorrection, MAX_CACHE_TICKS_AMNT},
    input::{
        InputBuffer, KeyBind, KeyBinds, KeyCodeEnum, HOLD_SPRINT_BIND, JUMP_BIND,
        MOVE_BACKWARD_BIND, MOVE_FORWARD_BIND, MOVE_LEFT_BIND, MOVE_RIGHT_BIND,
    },
};
use typename::TypeName;

/// Label for systems ordering.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum InputSet {
    First,
}

/// Client input movement event.
#[derive(Event, Debug)]
pub struct InputMovementInput {
    pub entity: Entity,
    pub up: bool,
    pub left: bool,
    pub right: bool,
    pub down: bool,
    pub pressed: bool,
    pub peer_data: Option<(Vec3, Vec3, u64)>,
}

/// Client input movement event.
#[derive(Event, Debug)]
pub struct SyncControllerInput {
    pub entity: Entity,
    pub sync: ControllerInput,
}

impl Default for InputMovementInput {
    fn default() -> Self {
        Self {
            entity: Entity::from_bits(0),
            up: false,
            left: false,
            right: false,
            down: false,
            pressed: false,
            peer_data: None,
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
    pub position: Vec3,
}

#[derive(Event, Default, Resource)]
pub struct LastPeerLookTransform {
    pub map: HashMap<ClientId, u64>,
}

pub(crate) fn apply_peer_sync_look_transform(
    mut events: EventReader<PeerSyncLookTransform>,
    mut query: Query<(&mut LookTransform, &mut Transform)>,
    mut last: ResMut<LastPeerLookTransform>,
    mut cache: ResMut<PhysicsCache>,
    stamp: Res<TickRateStamp>,
    mut priority: ResMut<PriorityPhysicsCache>,
) {
    for event in events.read() {
        let mut go = false;
        match last.map.get_mut(&event.handle) {
            Some(old_stamp) => {
                if event.stamp > *old_stamp {
                    *old_stamp = event.stamp;
                    go = true;
                }
            }
            None => {
                go = true;
                last.map.insert(event.handle, event.stamp);
            }
        }
        if go {
            match query.get_mut(event.entity) {
                Ok((mut l, mut t)) => {
                    l.target = event.target;
                    if stamp.large == event.stamp {
                        t.translation = event.position;
                    }
                }
                Err(_) => {
                    warn!("Couldnt find looktransform for sync.");
                }
            }
            if stamp.large != event.stamp {
                match cache.cache.get_mut(&event.stamp) {
                    Some(map) => match map.get_mut(&event.entity) {
                        Some(c) => {
                            c.transform.translation = event.position;
                        }
                        None => {
                            warn!(
                                "Missed peer position for looktransform 1. {:?}",
                                event.entity
                            );
                        }
                    },
                    None => {
                        warn!(
                            "Missed peer position for looktransform. {:?}:{} current tick: {}",
                            event.entity, event.stamp, stamp.large
                        );
                    }
                }
                match priority.cache.get_mut(&event.stamp) {
                    Some(map) => {
                        map.insert(event.entity, PriorityUpdate::Position(event.position));
                    }
                    None => {
                        let mut map = HashMap::new();
                        map.insert(event.entity, PriorityUpdate::Position(event.position));
                        priority.cache.insert(event.stamp, map);
                    }
                }
            }
        }
    }
}

#[derive(Resource, Default, Clone)]
pub struct RecordedControllerInput {
    pub input: HashMap<u64, Vec<RecordedInput>>,
}
#[derive(Resource, Default, Clone)]
pub struct PeerInputCache {
    pub reliable: HashMap<
        ClientId,
        HashMap<u64, Vec<IncomingReliableServerMessage<PeerReliableControllerMessage>>>,
    >,
    pub look_transform_syncs: HashMap<
        ClientId,
        HashMap<u64, HashMap<u8, IncomingUnreliableServerMessage<PeerUnreliableControllerMessage>>>,
    >,
    pub look_transform_best_ticks: HashMap<ClientId, (u64, u8)>,
}

pub(crate) fn process_peer_input(
    mut reliables_reader: EventReader<IncomingReliableServerMessage<PeerReliableControllerMessage>>,
    mut unreliables_reader: EventReader<
        IncomingUnreliableServerMessage<PeerUnreliableControllerMessage>,
    >,
    mut record: ResMut<RecordedControllerInput>,
    stamp: Res<TickRateStamp>,
    mut movement_input_event: EventWriter<InputMovementInput>,
    mut sync: EventWriter<PeerSyncLookTransform>,
    peer_pawns: Res<PeerPawns>,
    mut start_correction: EventWriter<StartCorrection>,
    mut sync_controller: EventWriter<SyncControllerInput>,
    mut input_cache: ResMut<PeerInputCache>,
    client: Res<RenetClient>,
    tickrate: Res<TickRate>,
    pawnid: Res<PawnId>,
) {
    let mut new_correction = false;
    let mut earliest_tick = 0;

    for r in reliables_reader.read() {
        let handle = ClientId::from_raw(r.message.peer_handle as u64);
        let large = stamp.calculate_large(r.message.client_stamp);
        match input_cache.reliable.get_mut(&handle) {
            Some(map) => match map.get_mut(&large) {
                Some(list) => {
                    list.push(r.clone());
                }
                None => {
                    map.insert(large, vec![r.clone()]);
                }
            },
            None => {
                let mut map = HashMap::new();
                map.insert(large, vec![r.clone()]);
                input_cache.reliable.insert(handle, map);
            }
        }
    }
    let latency_in_ticks = (client.rtt() as f32 / (1. / tickrate.fixed_rate as f32)).floor() as u64;
    let desired_tick = stamp.large - latency_in_ticks - latency_in_ticks;
    let mut reliables = vec![];
    for (_, reliable_cache) in input_cache.reliable.iter_mut() {
        for i in reliable_cache.clone().keys().sorted() {
            if i > &desired_tick {
                break;
            }
            for e in reliable_cache.get(i).unwrap() {
                reliables.push(e.clone());
            }
            reliable_cache.remove(i);
            break;
        }
    }

    for message in reliables.iter() {
        let large_client_stamp = stamp.calculate_large(message.message.client_stamp);

        let msg = RecordedInput::Reliable(message.message.clone());

        match record.input.get_mut(&large_client_stamp) {
            Some(v) => {
                v.push(msg);
            }
            None => {
                record.input.insert(large_client_stamp, vec![msg]);
            }
        }
        match &message.message.message {
            PeerControllerClientMessage::MovementInput(input, position, look_transform_target) => {
                match peer_pawns
                    .map
                    .get(&ClientId::from_raw(message.message.peer_handle.into()))
                {
                    Some(peer) => {
                        movement_input_event.send(InputMovementInput {
                            entity: *peer,
                            up: input.up,
                            left: input.left,
                            right: input.right,
                            down: input.down,
                            pressed: input.pressed,
                            peer_data: Some((
                                *position,
                                *look_transform_target,
                                large_client_stamp,
                            )),
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
            PeerControllerClientMessage::SyncControllerInput(input) => {
                match peer_pawns
                    .map
                    .get(&ClientId::from_raw(message.message.peer_handle.into()))
                {
                    Some(peer) => {
                        match pawnid.client {
                            Some(ce) => {
                                if *peer == ce {
                                    continue;
                                }
                            }
                            None => {}
                        }

                        sync_controller.send(SyncControllerInput {
                            entity: *peer,
                            sync: input.clone(),
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

    for u in unreliables_reader.read() {
        let client_id = ClientId::from_raw(u.message.peer_handle as u64);
        let large_client_stamp = stamp.calculate_large(u.message.client_stamp);
        match u.message.message {
            pawn::net::UnreliablePeerControllerClientMessage::UpdateLookTransform(
                _,
                _,
                new_sub,
            ) => {
                match input_cache.look_transform_best_ticks.get(&client_id) {
                    Some((old_best_tick, old_best_sub)) => {
                        if !(old_best_tick < &large_client_stamp
                            || (old_best_tick == &large_client_stamp && new_sub > *old_best_sub))
                        {
                            continue;
                        }
                    }
                    None => {}
                }

                input_cache
                    .look_transform_best_ticks
                    .insert(client_id, (large_client_stamp, new_sub));

                match input_cache.look_transform_syncs.get_mut(&client_id) {
                    Some(look_transform_ticks) => {
                        match look_transform_ticks.get_mut(&large_client_stamp) {
                            Some(v) => {
                                v.insert(new_sub, u.clone());
                            }
                            None => {
                                let mut map = HashMap::new();
                                map.insert(new_sub, u.clone());
                                look_transform_ticks.insert(large_client_stamp, map);
                            }
                        }
                    }
                    None => {
                        let mut map = HashMap::new();
                        map.insert(new_sub, u.clone());
                        let mut map2 = HashMap::new();
                        map2.insert(large_client_stamp, map);
                        input_cache.look_transform_syncs.insert(client_id, map2);
                    }
                }
            }
        }
    }

    for peer in input_cache.look_transform_syncs.clone().keys() {
        let mut best_tick_option = None;
        let peer_data = input_cache.look_transform_syncs.get_mut(peer).unwrap();

        match peer_data.get(&desired_tick) {
            Some(_) => {
                best_tick_option = Some(desired_tick);
            }
            None => {
                for i in peer_data.keys().sorted().rev() {
                    if i > &desired_tick {
                        continue;
                    }
                    best_tick_option = Some(*i);
                    break;
                }
            }
        }

        match best_tick_option {
            Some(best_tick) => {
                let steps = peer_data.get(&best_tick).unwrap();
                let mut best_sub = 0;

                if steps.keys().len() == 0 {
                    warn!("Empty substeps?");
                    continue;
                }
                for sub_step in steps.keys().sorted().rev() {
                    best_sub = *sub_step;
                    break;
                }

                let message = peer_data.get(&best_tick).unwrap().get(&best_sub).unwrap();

                let msg = RecordedInput::Unreliable(message.message.clone());
                match record.input.get_mut(&best_tick) {
                    Some(v) => {
                        v.push(msg);
                    }
                    None => {
                        record.input.insert(best_tick, vec![msg]);
                    }
                }
                match &message.message.message {
                    pawn::net::UnreliablePeerControllerClientMessage::UpdateLookTransform(
                        target,
                        position,
                        _,
                    ) => {
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
                                    position: *position,
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

                let mut clean_cache = vec![];
                let mut clean_steps = vec![];
                let bound_data = peer_data.clone();
                for i in bound_data.keys().sorted() {
                    if i < &best_tick {
                        clean_cache.push(i);
                    } else if i == &best_tick {
                        for sub in bound_data.get(i).unwrap().keys() {
                            if sub <= &best_sub {
                                clean_steps.push(*sub);
                            }
                        }
                    }
                }
                for i in clean_cache {
                    peer_data.remove(i);
                }
                let mut clean_last = false;
                match peer_data.get_mut(&best_tick) {
                    Some(c) => {
                        for s in clean_steps {
                            c.remove(&s);
                        }
                        clean_last = c.len() == 0;
                    }
                    None => {}
                }
                if clean_last {
                    peer_data.remove(&best_tick);
                }

                input_cache
                    .look_transform_best_ticks
                    .insert(*peer, (best_tick, best_sub));
            }
            None => {}
        }
    }
    // Doesnt send StartCorrect if peer input is for our exact tick or future tack.
    if new_correction && earliest_tick == stamp.large {
        info!("Perfect peer sync.");
    }
    if new_correction && earliest_tick < stamp.large {
        start_correction.send(StartCorrection {
            start_tick: earliest_tick,
            last_tick: stamp.large,
        });
    }
}
/// Client fn
pub(crate) fn sync_controller_input(
    mut events: EventReader<SyncControllerInput>,
    mut query: Query<&mut ControllerInput>,
) {
    for event in events.read() {
        match query.get_mut(event.entity) {
            Ok(mut controller_input) => {
                *controller_input = event.sync.clone();
            }
            Err(_) => {
                warn!("Couldnt find entity to sync for.");
            }
        }
    }
}

pub(crate) fn clean_recorded_input(
    mut record: ResMut<RecordedControllerInput>,
    stamp: Res<TickRateStamp>,
) {
    let mut to_remove = vec![];
    for recorded_stamp in record.input.keys() {
        if stamp.large >= MAX_CACHE_TICKS_AMNT
            && recorded_stamp < &(stamp.large - MAX_CACHE_TICKS_AMNT)
        {
            to_remove.push(*recorded_stamp);
        }
    }
    for i in to_remove {
        record.input.remove(&i);
    }
}
#[derive(Resource, Default)]
pub(crate) struct Pressed {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

/// Sends client input instantly from Update schedule.
pub(crate) fn send_client_input_to_server(
    keyboard: Res<Input<KeyCode>>,
    mut client: ResMut<RenetClient>,
    binds: Res<KeyBinds>,
    typenames: Res<Typenames>,
    stamp: Res<TickRateStamp>,
    start: Res<ClientStartedSyncing>,
    mut pressed: Local<Pressed>,
) {
    if !start.0 {
        return;
    }
    let mut inputs = vec![];
    if keyboard.just_pressed(binds.keyboard_bind(MOVE_FORWARD_BIND)) {
        pressed.up = true;
        inputs.push(ControllerClientMessage::MovementInput(MovementInput {
            up: true,
            pressed: true,
            ..Default::default()
        }));
    }
    if keyboard.just_pressed(binds.keyboard_bind(MOVE_BACKWARD_BIND)) {
        pressed.down = true;

        inputs.push(ControllerClientMessage::MovementInput(MovementInput {
            down: true,
            pressed: true,
            ..Default::default()
        }));
    }
    if keyboard.just_pressed(binds.keyboard_bind(MOVE_LEFT_BIND)) {
        pressed.left = true;

        inputs.push(ControllerClientMessage::MovementInput(MovementInput {
            left: true,
            pressed: true,
            ..Default::default()
        }));
    }
    if keyboard.just_pressed(binds.keyboard_bind(MOVE_RIGHT_BIND)) {
        pressed.right = true;

        inputs.push(ControllerClientMessage::MovementInput(MovementInput {
            right: true,
            pressed: true,
            ..Default::default()
        }));
    }

    if keyboard.just_released(binds.keyboard_bind(MOVE_FORWARD_BIND)) && pressed.up {
        inputs.push(ControllerClientMessage::MovementInput(MovementInput {
            up: true,
            pressed: false,
            ..Default::default()
        }));
    }
    if keyboard.just_released(binds.keyboard_bind(MOVE_BACKWARD_BIND)) && pressed.down {
        inputs.push(ControllerClientMessage::MovementInput(MovementInput {
            down: true,
            pressed: false,
            ..Default::default()
        }));
    }
    if keyboard.just_released(binds.keyboard_bind(MOVE_LEFT_BIND)) && pressed.left {
        inputs.push(ControllerClientMessage::MovementInput(MovementInput {
            left: true,
            pressed: false,
            ..Default::default()
        }));
    }
    if keyboard.just_released(binds.keyboard_bind(MOVE_RIGHT_BIND)) && pressed.right {
        inputs.push(ControllerClientMessage::MovementInput(MovementInput {
            right: true,
            pressed: false,
            ..Default::default()
        }));
    }

    let id = typenames
        .reliable_net_types
        .get(&ControllerClientMessage::type_name())
        .unwrap();
    let mut messages = vec![];
    for input in inputs.iter() {
        messages.push(ReliableMessage {
            serialized: bincode::serialize(input).unwrap(),
            typename_net: *id,
        });
    }
    if messages.len() > 0 {
        client.send_message(
            RENET_RELIABLE_ORDERED_ID,
            bincode::serialize(&ReliableClientMessageBatch {
                messages,
                stamp: stamp.tick,
                not_timed: true,
            })
            .unwrap(),
        );
    }
}

pub(crate) fn get_client_input(
    keyboard: Res<InputBuffer>,
    mut movement_event: EventWriter<InputMovementInput>,
    pawn_id: Res<PawnId>,
    mut pressed: Local<Pressed>,
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
        pressed.up = true;
        movement_event.send(InputMovementInput {
            entity: pawn_entity,
            up: true,
            pressed: true,
            ..Default::default()
        });
    }
    if keyboard.just_pressed(MOVE_BACKWARD_BIND) {
        pressed.down = true;

        movement_event.send(InputMovementInput {
            entity: pawn_entity,
            down: true,
            pressed: true,
            ..Default::default()
        });
    }
    if keyboard.just_pressed(MOVE_LEFT_BIND) {
        pressed.left = true;

        movement_event.send(InputMovementInput {
            entity: pawn_entity,
            left: true,
            pressed: true,
            ..Default::default()
        });
    }
    if keyboard.just_pressed(MOVE_RIGHT_BIND) {
        pressed.right = true;

        movement_event.send(InputMovementInput {
            entity: pawn_entity,
            right: true,
            pressed: true,
            ..Default::default()
        });
    }

    if keyboard.just_released(MOVE_FORWARD_BIND) && pressed.up {
        movement_event.send(InputMovementInput {
            entity: pawn_entity,
            up: true,
            pressed: false,
            ..Default::default()
        });
    }
    if keyboard.just_released(MOVE_BACKWARD_BIND) && pressed.down {
        movement_event.send(InputMovementInput {
            entity: pawn_entity,
            down: true,
            pressed: false,
            ..Default::default()
        });
    }
    if keyboard.just_released(MOVE_LEFT_BIND) && pressed.left {
        movement_event.send(InputMovementInput {
            entity: pawn_entity,
            left: true,
            pressed: false,
            ..Default::default()
        });
    }
    if keyboard.just_released(MOVE_RIGHT_BIND) && pressed.right {
        movement_event.send(InputMovementInput {
            entity: pawn_entity,
            right: true,
            pressed: false,
            ..Default::default()
        });
    }
}
/// Label for systems ordering.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]

pub enum ControllerSet {
    Input,
}

/// Manage controller input for humanoid. The controller can be controlled by a player or AI.
pub(crate) fn controller_input(
    mut humanoids_query: Query<(&mut ControllerInput, &mut LookTransform, &mut Transform)>,
    mut movement_input_event: EventReader<InputMovementInput>,
    mut cache: ResMut<PhysicsCache>,
    mut look_cache: ResMut<LookTransformCache>,
    stampres: Res<TickRateStamp>,
    mut priority: ResMut<PriorityPhysicsCache>,
) {
    for new_event in movement_input_event.read() {
        let player_entity = new_event.entity;

        let player_input_component_result = humanoids_query.get_mut(player_entity);

        match player_input_component_result {
            Ok((mut player_input_component, mut look_transform, mut transform)) => {
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

                match new_event.peer_data {
                    Some((position, look_target, stamp)) => {
                        look_transform.target = look_target;

                        if stamp == stampres.large {
                            transform.translation = position;
                        } else {
                            match look_cache.cache.get_mut(&stamp) {
                                Some(map) => match map.get_mut(&player_entity) {
                                    Some(l) => {
                                        l.target = look_target;
                                    }
                                    None => {
                                        warn!("Missed look cache. 0");
                                    }
                                },
                                None => {
                                    warn!("Missed look cache.");
                                }
                            }
                            match priority.cache.get_mut(&stamp) {
                                Some(map) => {
                                    map.insert(player_entity, PriorityUpdate::Position(position));
                                }
                                None => {
                                    let mut map = HashMap::new();
                                    map.insert(player_entity, PriorityUpdate::Position(position));
                                    priority.cache.insert(stamp, map);
                                }
                            }
                            match cache.cache.get_mut(&stamp) {
                                Some(map) => match map.get_mut(&player_entity) {
                                    Some(c) => {
                                        c.transform.translation = position;
                                    }
                                    None => {
                                        warn!("Missed physics cache1.");
                                    }
                                },
                                None => {
                                    warn!("Missed physics cache.");
                                }
                            }
                        }
                    }
                    None => {}
                }
            }
            Err(_rr) => {
                warn!("Couldn't process player input (movement_input_event): couldn't find player_entity 0. {:?}", player_entity);
            }
        }
    }
}
