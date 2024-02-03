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
    client::{IncomingReliableServerMessage, IncomingUnreliableServerMessage, TickLatency},
    messaging::{ReliableClientMessageBatch, ReliableMessage, Typenames},
    plugin::RENET_RELIABLE_ORDERED_ID,
    stamp::TickRateStamp,
};
use pawn::net::{PeerUpdateLookTransform, UnreliablePeerControllerClientMessage};
use physics::{cache::PhysicsCache, sync::ClientStartedSyncing};
use resources::{
    correction::{StartCorrection, MAX_CACHE_TICKS_AMNT},
    input::{
        KeyBind, KeyBinds, KeyCodeEnum, HOLD_SPRINT_BIND, JUMP_BIND, MOVE_BACKWARD_BIND,
        MOVE_FORWARD_BIND, MOVE_LEFT_BIND, MOVE_RIGHT_BIND,
    },
    modes::is_server,
    pawn::ClientPawn,
    physics::{PriorityPhysicsCache, PriorityUpdate},
};
use typename::TypeName;

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
#[derive(Event, Debug)]
pub struct PeerSyncLookTransform {
    pub entity: Entity,
    pub target: Vec3,
    pub handle: ClientId,
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

pub(crate) fn cache_peer_sync_look_transform(
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
                    Some(c) => match c.get_mut(&event.server_stamp) {
                        Some(cache_data) => {
                            cache_data.target = event.target;
                        }
                        None => {
                            let mut cache_data = default_look_transform();
                            cache_data.target = event.target;
                            c.insert(event.server_stamp, cache_data);
                        }
                    },
                    None => {
                        let mut m = HashMap::new();
                        let mut cache_data = default_look_transform();
                        cache_data.target = event.target;
                        m.insert(event.server_stamp, cache_data);
                        look_cache.cache.insert(event.entity, m);
                    }
                }
                if stamp.large == event.server_stamp {
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
                        warn!(
                            "Missed peer position for looktransform 1. {:?}",
                            event.entity
                        );
                    }
                },
                None => {
                    warn!(
                        "Missed peer position for looktransform. {:?}:{} current tick: {}",
                        event.entity, adjusted_stamp, stamp.large
                    );
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
    server_stamp: u64,
    client_sub_id: u8,
    correct: bool,
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
    mut look_update_queue: Local<HashMap<ClientId, HashMap<u64, HashMap<u8, LookTick>>>>,

    latency: Res<TickLatency>,
) {
    let mut new_correction = false;
    let mut earliest_tick = 0;

    for message in reliables_reader.read() {
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
                            peer_data: Some((*position, *look_transform_target, message.stamp)),
                        });
                        new_correction = true;
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

    let mut ordered_unreliables: HashMap<u64, HashMap<u8, LookTick>> = HashMap::new();
    for u in unreliables_reader.read() {
        let client_id = ClientId::from_raw(u.message.peer_handle as u64);

        match &u.message.message {
            UnreliablePeerControllerClientMessage::UpdateLookTransform(update) => {
                let up: LookTick = LookTick {
                    update: update.clone(),
                    peer_handle: client_id,
                    server_stamp: u.stamp,
                    client_sub_id: update.sub_tick,
                    correct: true,
                };

                match ordered_unreliables.get_mut(&u.stamp) {
                    Some(m) => {
                        m.insert(update.sub_tick, up);
                    }
                    None => {
                        let mut m = HashMap::new();
                        m.insert(update.sub_tick, up);
                        ordered_unreliables.insert(u.stamp, m);
                    }
                }
            }
        }
    }

    let latency_in_ticks = latency.latency as u64;
    let desired_tick = stamp.large - latency_in_ticks;

    for server_stamp in ordered_unreliables.keys().sorted().rev() {
        let subs = ordered_unreliables.get(server_stamp).unwrap();
        for sub in subs.keys().sorted().rev() {
            let mut up = subs.get(sub).unwrap().clone();
            if *server_stamp == desired_tick {
                up.correct = true;
            }
            let client_id = up.peer_handle;
            let mut latest = false;

            match look_update_queue.get_mut(&client_id) {
                Some(q1) => match q1.get_mut(&up.server_stamp) {
                    Some(q2) => {
                        for i in q2.keys().sorted().rev() {
                            if up.client_sub_id > *i {
                                latest = true;
                            }
                            break;
                        }
                        q2.insert(up.client_sub_id, up.clone());
                    }
                    None => {
                        let mut m = HashMap::new();
                        m.insert(up.client_sub_id, up.clone());
                        q1.insert(up.server_stamp, m);
                        latest = true;
                    }
                },
                None => {
                    let mut n = HashMap::new();
                    n.insert(up.client_sub_id, up.clone());
                    let mut m = HashMap::new();
                    m.insert(up.server_stamp, n);
                    look_update_queue.insert(client_id, m);
                    latest = true;
                }
            }
            if !latest {
                continue;
            }
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

    for (_, updates) in input_cache.look_transform_best_ticks.iter() {
        for update in updates.iter() {
            match peer_pawns.map.get(&update.peer_handle) {
                Some(peer) => {
                    let msg = PeerSyncLookTransform {
                        entity: *peer,
                        target: update.update.target,
                        handle: update.peer_handle,
                        position: update.update.position,
                        server_stamp: update.server_stamp,
                    };
                    //info!("debug0 at tick {}: {:?}", stamp.large, msg);

                    sync.send(msg);

                    new_correction = true;

                    let e = update.server_stamp - 1;
                    if e < earliest_tick || earliest_tick == 0 {
                        earliest_tick = e;
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
    for (_, cache) in look_update_queue.iter_mut() {
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
    mut cache: ResMut<ControllerCache>,
    stamp: Res<TickRateStamp>,
) {
    for event in events.read() {
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
        info!(
            "Synced controller input {:?} , serverstamp{} at {}",
            event.sync, event.server_stamp, stamp.large
        );
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
pub(crate) fn keyboard_input(
    keyboard: Res<Input<KeyCode>>,
    mut client: ResMut<RenetClient>,
    binds: Res<KeyBinds>,
    typenames: Res<Typenames>,
    stamp: Res<TickRateStamp>,
    start: Res<ClientStartedSyncing>,
    mut pressed: Local<Pressed>,
    pawn_id: Res<PawnId>,
    mut movement_event: EventWriter<InputMovementInput>,
    mut i: Local<u64>,
) {
    *i += 1;
    if !start.0 {
        return;
    }
    let pawn_entity;
    match pawn_id.client {
        Some(i) => {
            pawn_entity = i;
        }
        None => {
            return;
        }
    }
    let mut inputs = vec![];
    if keyboard.just_pressed(binds.keyboard_bind(MOVE_FORWARD_BIND)) && !pressed.up {
        pressed.up = true;
        inputs.push(ControllerClientMessage::MovementInput(MovementInput {
            up: true,
            pressed: true,
            ..Default::default()
        }));
        movement_event.send(InputMovementInput {
            entity: pawn_entity,
            up: true,
            pressed: true,
            ..Default::default()
        });
    }
    if keyboard.just_pressed(binds.keyboard_bind(MOVE_BACKWARD_BIND)) && !pressed.down {
        pressed.down = true;

        inputs.push(ControllerClientMessage::MovementInput(MovementInput {
            down: true,
            pressed: true,
            ..Default::default()
        }));
        movement_event.send(InputMovementInput {
            entity: pawn_entity,
            down: true,
            pressed: true,
            ..Default::default()
        });
    }
    if keyboard.just_pressed(binds.keyboard_bind(MOVE_LEFT_BIND)) && !pressed.left {
        pressed.left = true;

        inputs.push(ControllerClientMessage::MovementInput(MovementInput {
            left: true,
            pressed: true,
            ..Default::default()
        }));
        movement_event.send(InputMovementInput {
            entity: pawn_entity,
            left: true,
            pressed: true,
            ..Default::default()
        });
    }
    if keyboard.just_pressed(binds.keyboard_bind(MOVE_RIGHT_BIND)) && !pressed.right {
        pressed.right = true;

        inputs.push(ControllerClientMessage::MovementInput(MovementInput {
            right: true,
            pressed: true,
            ..Default::default()
        }));

        movement_event.send(InputMovementInput {
            entity: pawn_entity,
            right: true,
            pressed: true,
            ..Default::default()
        });
    }

    if keyboard.just_released(binds.keyboard_bind(MOVE_FORWARD_BIND)) && pressed.up {
        pressed.up = false;
        inputs.push(ControllerClientMessage::MovementInput(MovementInput {
            up: true,
            pressed: false,
            ..Default::default()
        }));
        movement_event.send(InputMovementInput {
            entity: pawn_entity,
            up: true,
            pressed: false,
            ..Default::default()
        });
    }
    if keyboard.just_released(binds.keyboard_bind(MOVE_BACKWARD_BIND)) && pressed.down {
        pressed.down = false;
        movement_event.send(InputMovementInput {
            entity: pawn_entity,
            down: true,
            pressed: false,
            ..Default::default()
        });
        inputs.push(ControllerClientMessage::MovementInput(MovementInput {
            down: true,
            pressed: false,
            ..Default::default()
        }));
    }
    if keyboard.just_released(binds.keyboard_bind(MOVE_LEFT_BIND)) && pressed.left {
        pressed.left = false;
        inputs.push(ControllerClientMessage::MovementInput(MovementInput {
            left: true,
            pressed: false,
            ..Default::default()
        }));
        movement_event.send(InputMovementInput {
            entity: pawn_entity,
            left: true,
            pressed: false,
            ..Default::default()
        });
    }
    if keyboard.just_released(binds.keyboard_bind(MOVE_RIGHT_BIND)) && pressed.right {
        pressed.right = false;
        inputs.push(ControllerClientMessage::MovementInput(MovementInput {
            right: true,
            pressed: false,
            ..Default::default()
        }));
        movement_event.send(InputMovementInput {
            entity: pawn_entity,
            right: true,
            pressed: false,
            ..Default::default()
        });
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
        let large_target_tick = stamp.large + 1;

        let target_tick = TickRateStamp::new(large_target_tick).tick;
        client.send_message(
            RENET_RELIABLE_ORDERED_ID,
            bincode::serialize(&ReliableClientMessageBatch {
                messages,
                stamp: target_tick,
                not_timed: true,
            })
            .unwrap(),
        );
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
        match humanoids_query.get_mut(player_entity) {
            Ok((mut player_input_component, mut look_transform, mut transform)) => {
                let mut processed_input = ControllerInput::default();

                if new_event.peer_data.is_some() {
                    match controller_cache.cache.get(&player_entity) {
                        Some(c) => {
                            for i in c.keys().sorted().rev() {
                                processed_input = c.get(i).unwrap().clone();
                                break;
                            }
                        }
                        None => {}
                    }
                }

                if new_event.peer_data.is_none() {
                    processed_input = player_input_component.clone();
                }

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
                    Some((position, look_target, server_stamp)) => {
                        input_stamp = server_stamp;
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
                            Some(c) => match c.get_mut(&server_stamp) {
                                Some(l) => {
                                    l.target = look_target;
                                }
                                None => {
                                    let mut l = default_look_transform();
                                    l.target = look_target;
                                    c.insert(server_stamp, l);
                                }
                            },
                            None => {
                                let mut m = HashMap::new();
                                let mut l = default_look_transform();
                                l.target = look_target;
                                m.insert(server_stamp, l);
                                look_cache.cache.insert(player_entity, m);
                            }
                        }
                        if server_stamp == stampres.large {
                            look_transform.target = look_target;
                            *player_input_component = processed_input.clone();
                            transform.translation = position;
                            info!("perfect peer server stamp.");
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
                if !is_server() {
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
            }
            Err(_rr) => {
                warn!("Couldn't process player input (movement_input_event): couldn't find player_entity 0. {:?}", player_entity);
            }
        }
    }
}
