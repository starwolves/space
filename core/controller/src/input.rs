use std::collections::HashMap;

use crate::{
    controller::{ControllerCache, ControllerInput},
    net::{ControllerClientMessage, MovementInput, PeerControllerClientMessage},
    networking::{PeerReliableControllerMessage, PeerUnreliableControllerMessage},
};
use bevy::{
    ecs::{query::Without, system::Local},
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
use pawn::net::{PeerUpdateLookTransform, UnreliablePeerControllerClientMessage};
use physics::{cache::PhysicsCache, sync::ClientStartedSyncing};
use resources::{
    correction::{StartCorrection, MAX_CACHE_TICKS_AMNT},
    input::{
        InputBuffer, KeyBind, KeyBinds, KeyCodeEnum, HOLD_SPRINT_BIND, JUMP_BIND,
        MOVE_BACKWARD_BIND, MOVE_FORWARD_BIND, MOVE_LEFT_BIND, MOVE_RIGHT_BIND,
    },
    pawn::ClientPawn,
    physics::{PriorityPhysicsCache, PriorityUpdate},
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
    pub peer_data: Option<(Vec3, Vec3, u64, u64)>,
}

/// Client input movement event.
#[derive(Event, Debug)]
pub struct SyncControllerInput {
    pub entity: Entity,
    pub sync: ControllerInput,
    pub server_stamp: u64,
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
    pub client_stamp: u64,
    pub server_stamp: u64,
    pub position: Vec3,
}

#[derive(Event, Default, Resource)]
pub struct LastPeerLookTransform {
    pub map: HashMap<ClientId, u64>,
}

pub fn default_look_transform() -> LookTransform {
    LookTransform::new(
        Vec3::new(0., 1.8 - R, 0.),
        Vec3::new(0., 1.8 - R, -2.),
        Vec3::Y,
    )
}
pub const R: f32 = 0.5;

pub(crate) fn apply_peer_sync_look_transform(
    mut events: EventReader<PeerSyncLookTransform>,
    mut query: Query<(&mut LookTransform, &mut Transform)>,
    mut cache: ResMut<PhysicsCache>,
    stamp: Res<TickRateStamp>,
    mut priority: ResMut<PriorityPhysicsCache>,
    mut look_cache: ResMut<LookTransformCache>,
) {
    for event in events.read() {
        match query.get_mut(event.entity) {
            Ok((mut l, mut t)) => {
                match look_cache.cache.get_mut(&event.entity) {
                    Some(c) => match c.get_mut(&event.client_stamp) {
                        Some(l) => {
                            l.target = event.target;
                        }
                        None => {
                            let mut l = default_look_transform();
                            l.target = event.target;
                            c.insert(event.client_stamp, l);
                        }
                    },
                    None => {
                        let mut m = HashMap::new();
                        let mut l = default_look_transform();
                        l.target = event.target;
                        m.insert(event.client_stamp, l);
                        look_cache.cache.insert(event.entity, m);
                    }
                }
                if stamp.large == event.client_stamp {
                    l.target = event.target;
                }
                if stamp.large == event.server_stamp {
                    t.translation = event.position;
                }
            }
            Err(_) => {
                warn!("Couldnt find looktransform for sync.");
            }
        }
        let adjusted_stamp = event.server_stamp - 1;

        if stamp.large != event.server_stamp {
            match cache.cache.get_mut(&adjusted_stamp) {
                Some(map) => match map.get_mut(&event.entity) {
                    Some(c) => {
                        c.transform.translation = event.position;
                    }
                    None => {
                        /*warn!(
                            "Missed peer position for looktransform 1. {:?}",
                            event.entity
                        );*/
                    }
                },
                None => {
                    /*warn!(
                        "Missed peer position for looktransform. {:?}:{} current tick: {}",
                        event.entity, adjusted_stamp, stamp.large
                    );*/
                }
            }
        }
        match priority.cache.get_mut(&adjusted_stamp) {
            Some(map) => {
                map.insert(event.entity, PriorityUpdate::Position(event.position));
            }
            None => {
                let mut map = HashMap::new();
                map.insert(event.entity, PriorityUpdate::Position(event.position));
                priority.cache.insert(adjusted_stamp, map);
            }
        }
    }
}

#[derive(Resource, Default, Clone)]
pub struct PeerInputCache {
    pub reliable: HashMap<
        ClientId,
        HashMap<u64, Vec<IncomingReliableServerMessage<PeerReliableControllerMessage>>>,
    >,
    pub look_transform_best_ticks: HashMap<ClientId, Vec<LookTick>>,
}
#[derive(Resource, Clone)]
pub struct LookTick {
    update: PeerUpdateLookTransform,
    peer_handle: ClientId,
    client_stamp: u64,
    server_stamp: u64,
    start_correction: bool,
}

pub(crate) fn process_peer_input(
    mut reliables_reader: EventReader<IncomingReliableServerMessage<PeerReliableControllerMessage>>,
    mut unreliables_reader: EventReader<
        IncomingUnreliableServerMessage<PeerUnreliableControllerMessage>,
    >,
    stamp: Res<TickRateStamp>,
    mut movement_input_event: EventWriter<InputMovementInput>,
    mut sync: EventWriter<PeerSyncLookTransform>,
    peer_pawns: Res<PeerPawns>,
    mut start_correction: EventWriter<StartCorrection>,
    mut sync_controller: EventWriter<SyncControllerInput>,
    mut input_cache: ResMut<PeerInputCache>,
    pawnid: Res<PawnId>,
    mut queue: Local<HashMap<ClientId, HashMap<u64, HashMap<u8, LookTick>>>>,
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
    let desired_tick = stamp.large;

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
                                message.stamp,
                            )),
                        });
                        new_correction = true;
                        let e = stamp.calculate_large(message.message.client_stamp);
                        if e < earliest_tick || earliest_tick == 0 {
                            earliest_tick = e;
                        }
                        let e = message.stamp - 1;
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
                            server_stamp: message.stamp,
                        });

                        new_correction = true;
                        let e = stamp.calculate_large(message.message.client_stamp);
                        if e < earliest_tick || earliest_tick == 0 {
                            earliest_tick = e;
                        }
                        let e = message.stamp - 1;
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
        match &u.message.message {
            UnreliablePeerControllerClientMessage::UpdateLookTransform(update) => {
                let mut latest = false;

                let up = LookTick {
                    update: update.clone(),
                    peer_handle: client_id,
                    client_stamp: large_client_stamp,
                    server_stamp: u.stamp,
                    start_correction: false,
                };
                match queue.get_mut(&client_id) {
                    Some(q1) => match q1.get_mut(&u.stamp) {
                        Some(q2) => {
                            q2.insert(update.sub_tick, up.clone());
                            for i in q2.keys().sorted().rev() {
                                if update.sub_tick > *i {
                                    latest = true;
                                }
                                break;
                            }
                        }
                        None => {
                            let mut m = HashMap::new();
                            m.insert(update.sub_tick, up.clone());
                            q1.insert(u.stamp, m);
                            latest = true;
                        }
                    },
                    None => {
                        let mut n = HashMap::new();
                        n.insert(update.sub_tick, up.clone());
                        let mut m = HashMap::new();
                        m.insert(u.stamp, n);
                        queue.insert(client_id, m);
                        latest = true;
                    }
                }
                if !latest {
                    continue;
                }
                // This shouldnt trigger a StartCorrection
                match input_cache.look_transform_best_ticks.get_mut(&client_id) {
                    Some(v) => {
                        v.push(up);
                    }
                    None => {
                        input_cache
                            .look_transform_best_ticks
                            .insert(client_id, vec![up]);
                    }
                }
            }
        }
    }

    for (client, netcode_updates) in queue.iter() {
        match netcode_updates.get(&desired_tick) {
            Some(ups) => {
                for i in ups.keys().sorted().rev() {
                    let mut look_tick = ups.get(i).unwrap().clone();
                    look_tick.start_correction = true;
                    match input_cache.look_transform_best_ticks.get_mut(client) {
                        Some(bests) => {
                            bests.push(look_tick);
                        }
                        None => {
                            input_cache
                                .look_transform_best_ticks
                                .insert(*client, vec![look_tick]);
                        }
                    }

                    break;
                }
            }
            None => {}
        }
    }

    for (_, updates) in input_cache.look_transform_best_ticks.iter() {
        for update in updates.iter() {
            match peer_pawns.map.get(&update.peer_handle) {
                Some(peer) => {
                    let e = update.client_stamp;
                    sync.send(PeerSyncLookTransform {
                        entity: *peer,
                        target: update.update.target,
                        handle: update.peer_handle,
                        client_stamp: e,
                        position: update.update.position,
                        server_stamp: update.server_stamp,
                    });

                    /*info!(
                        "process_peer_input client stamp {} server stamp {} subid {}",
                        e, update.server_stamp, update.update.sub_tick
                    );*/

                    if update.start_correction {
                        new_correction = true;

                        if e < earliest_tick || earliest_tick == 0 {
                            earliest_tick = e;
                        }
                        let e = update.server_stamp - 1;
                        if e < earliest_tick || earliest_tick == 0 {
                            earliest_tick = e;
                        }
                    }
                }
                None => {
                    warn!("Couldnt find peer pawn 2. {}", update.peer_handle);
                }
            }
        }
    }

    input_cache.look_transform_best_ticks.clear();

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
    // Clean cache.
    for (_, cache) in queue.iter_mut() {
        if cache.len() > MAX_CACHE_TICKS_AMNT as usize {
            let mut j = 0;
            for i in cache.clone().keys().sorted().rev() {
                if j >= MAX_CACHE_TICKS_AMNT {
                    cache.remove(i);
                }
                j += 1;
            }
        }
    }
}
/// Client fn
pub(crate) fn sync_controller_input(
    mut events: EventReader<SyncControllerInput>,
    mut query: Query<&mut ControllerInput>,
    mut cache: ResMut<ControllerCache>,
) {
    for event in events.read() {
        match query.get_mut(event.entity) {
            Ok(mut controller_input) => {
                *controller_input = event.sync.clone();
                match cache.cache.get_mut(&event.entity) {
                    Some(c) => {
                        c.insert(event.server_stamp, event.sync.clone());
                    }
                    None => {
                        let mut map = HashMap::new();
                        map.insert(event.server_stamp, event.sync.clone());
                        cache.cache.insert(event.entity, map);
                    }
                }
            }
            Err(_) => {
                warn!("Couldnt find entity to sync for.");
            }
        }
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

pub(crate) fn apply_controller_cache_to_peers(
    cache: Res<ControllerCache>,
    mut query: Query<(Entity, &mut ControllerInput), Without<ClientPawn>>,
    stamp: Res<TickRateStamp>,
) {
    for (entity, mut input_component) in query.iter_mut() {
        match cache.cache.get(&entity) {
            Some(input_cache) => {
                for i in input_cache.keys().sorted().rev() {
                    if i > &stamp.large {
                        continue;
                    }
                    let input = input_cache.get(i).unwrap();
                    *input_component = input.clone();
                    break;
                }
            }
            None => {}
        }
    }
}

/// Manage controller input for humanoid. The controller can be controlled by a player or AI.
pub(crate) fn controller_input(
    mut humanoids_query: Query<(&mut ControllerInput, &mut LookTransform, &mut Transform)>,
    mut movement_input_event: EventReader<InputMovementInput>,
    mut cache: ResMut<PhysicsCache>,
    mut look_cache: ResMut<LookTransformCache>,
    stampres: Res<TickRateStamp>,
    mut priority: ResMut<PriorityPhysicsCache>,
    mut controller_cache: ResMut<ControllerCache>,
) {
    for new_event in movement_input_event.read() {
        let player_entity = new_event.entity;

        let player_input_component_result = humanoids_query.get_mut(player_entity);

        let mut processed_input = ControllerInput::default();

        match controller_cache.cache.get(&player_entity) {
            Some(c) => {
                for i in c.keys().sorted().rev() {
                    processed_input = c.get(i).unwrap().clone();
                    break;
                }
            }
            None => {}
        }

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

                processed_input.movement_vector += additive;

                let input_stamp;

                match new_event.peer_data {
                    Some((position, look_target, client_stamp, server_stamp)) => {
                        input_stamp = client_stamp;
                        let adjusted_stamp = server_stamp - 1;

                        match priority.cache.get_mut(&adjusted_stamp) {
                            Some(map) => {
                                map.insert(player_entity, PriorityUpdate::Position(position));
                            }
                            None => {
                                let mut map = HashMap::new();
                                map.insert(player_entity, PriorityUpdate::Position(position));
                                priority.cache.insert(adjusted_stamp, map);
                            }
                        }
                        match look_cache.cache.get_mut(&player_entity) {
                            Some(c) => match c.get_mut(&client_stamp) {
                                Some(l) => {
                                    l.target = look_target;
                                }
                                None => {
                                    let mut l = default_look_transform();
                                    l.target = look_target;
                                    c.insert(client_stamp, l);
                                }
                            },
                            None => {
                                let mut m = HashMap::new();
                                let mut l = default_look_transform();
                                l.target = look_target;
                                m.insert(client_stamp, l);
                                look_cache.cache.insert(player_entity, m);
                            }
                        }
                        if client_stamp == stampres.large {
                            look_transform.target = look_target;
                            *player_input_component = processed_input.clone();
                        }
                        if server_stamp == stampres.large {
                            transform.translation = position;
                        }
                        match cache.cache.get_mut(&adjusted_stamp) {
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
                    None => {
                        input_stamp = stampres.large;
                        *player_input_component = processed_input.clone();
                    }
                }

                match controller_cache.cache.get_mut(&player_entity) {
                    Some(map) => {
                        map.insert(input_stamp, processed_input.clone());
                    }
                    None => {
                        let mut map = HashMap::new();
                        map.insert(input_stamp, processed_input.clone());
                        controller_cache.cache.insert(player_entity, map);
                    }
                }
            }
            Err(_rr) => {
                warn!("Couldn't process player input (movement_input_event): couldn't find player_entity 0. {:?}", player_entity);
            }
        }
    }
}
