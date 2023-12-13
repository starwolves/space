use std::collections::HashMap;

use bevy::ecs::system::{ResMut, Resource};
use bevy::log::warn;
use bevy::{
    prelude::{EventReader, Local, Query, Res, SystemSet, Vec3},
    time::Time,
};

use bevy_renet::renet::{ClientId, RenetClient};
use bevy_xpbd_3d::prelude::{Physics, PhysicsTime};
use cameras::{controllers::fps::ActiveCamera, LookTransform};
use entity::spawn::PawnId;
use networking::messaging::{Typenames, UnreliableClientMessageBatch, UnreliableMessage};
use networking::plugin::RENET_UNRELIABLE_CHANNEL_ID;
use networking::server::{HandleToEntity, IncomingUnreliableClientMessage};
use networking::stamp::TickRateStamp;
use typename::TypeName;

use crate::net::UnreliableControllerClientMessage;

/// Label for systems ordering.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum LookTransformSet {
    Sync,
}
#[derive(Resource, Default)]
pub struct ServerMouseInputStamps {
    pub map: HashMap<ClientId, (u64, u8)>,
}
#[derive(Resource, Default)]
pub struct MouseInputStamps {
    pub i: u8,
}
impl MouseInputStamps {
    pub fn step(&mut self) {
        if self.i == u8::MAX {
            self.i = 0;
        } else {
            self.i += 1;
        }
    }
}
pub(crate) fn clear_mouse_stamps(mut mouse_stamps: ResMut<MouseInputStamps>) {
    mouse_stamps.i = 0;
}
pub(crate) fn clear_mouse_stamps_server(mut mouse_stamps: ResMut<ServerMouseInputStamps>) {
    mouse_stamps.map.clear();
}
pub(crate) fn client_sync_look_transform(
    mut look_transform_query: Query<&mut LookTransform>,
    mut client: ResMut<RenetClient>,
    mut prev_target: Local<Vec3>,
    state: Res<ActiveCamera>,
    pawn_id: Res<PawnId>,
    physics_loop: Res<Time<Physics>>,
    stamp: Res<TickRateStamp>,
    typenames: Res<Typenames>,
    mut mouse_stamps: ResMut<MouseInputStamps>,
) {
    let camera_entity;
    match state.option {
        Some(cam_ent) => {
            camera_entity = cam_ent;
        }
        None => {
            return;
        }
    }
    let lk;
    match look_transform_query.get(camera_entity) {
        Ok(look_transform) => {
            let difference = (*prev_target - look_transform.target).abs();
            if difference.length() > 0.0001 && !physics_loop.is_paused() {
                let id = typenames
                    .unreliable_net_types
                    .get(&UnreliableControllerClientMessage::type_name())
                    .unwrap();
                client.send_message(
                    RENET_UNRELIABLE_CHANNEL_ID,
                    bincode::serialize(&UnreliableClientMessageBatch {
                        messages: vec![UnreliableMessage {
                            serialized: bincode::serialize(
                                &UnreliableControllerClientMessage::UpdateLookTransform(
                                    look_transform.target,
                                    mouse_stamps.i,
                                ),
                            )
                            .unwrap(),
                            typename_net: *id,
                        }],
                        stamp: stamp.tick,
                        sub_step: true,
                    })
                    .unwrap(),
                );
                mouse_stamps.step();

                *prev_target = look_transform.target;
            }
            lk = look_transform.clone();
        }
        Err(_) => {
            warn!("Couldnt find camera component.");
            return;
        }
    }

    match pawn_id.client {
        Some(pawn_entity) => match look_transform_query.get_mut(pawn_entity) {
            Ok(mut look) => {
                *look = lk;
            }
            Err(_) => {
                warn!("Couldnt find mirrored LookTransform component.");
            }
        },
        None => {}
    }
}

pub(crate) fn server_sync_look_transform(
    mut humanoids: Query<&mut LookTransform>,
    mut messages: EventReader<IncomingUnreliableClientMessage<UnreliableControllerClientMessage>>,
    handle_to_entity: Res<HandleToEntity>,
    mut mouse_stamps: ResMut<ServerMouseInputStamps>,
) {
    let mut apply_look_transform = None;
    for msg in messages.read() {
        match msg.message {
            UnreliableControllerClientMessage::UpdateLookTransform(target, id) => {
                match mouse_stamps.map.get_mut(&msg.handle) {
                    Some((tick, sub)) => {
                        if msg.stamp < *tick || (msg.stamp == *tick && id < *sub) {
                            continue;
                        }
                        *sub = id;
                        *tick = msg.stamp;
                        apply_look_transform = Some((msg.handle, target));
                    }
                    None => {
                        mouse_stamps.map.insert(msg.handle, (msg.stamp, id));
                        apply_look_transform = Some((msg.handle, target));
                    }
                }
            }
        }
    }

    match apply_look_transform {
        Some((handle, target)) => match handle_to_entity.map.get(&handle) {
            Some(entity) => match humanoids.get_mut(*entity) {
                Ok(mut look_transform) => {
                    look_transform.target = target;
                }
                Err(_) => {
                    warn!("Couldnt find client entity components.");
                }
            },
            None => {
                warn!("Couldnt find handle entity.");
            }
        },
        None => {}
    }
}
