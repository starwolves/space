use bevy::prelude::{warn, EventReader, EventWriter, Local, Query, Res, SystemSet, Vec3};
use cameras::{controllers::fps::ActiveCamera, LookTransform};
use entity::spawn::PawnId;
use networking::{
    client::OutgoingReliableClientMessage,
    server::{HandleToEntity, IncomingReliableClientMessage},
};

use crate::net::PawnClientMessage;

/// Label for systems ordering.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum LookTransformSet {
    Sync,
}

pub(crate) fn client_sync_look_transform(
    mut look_transform_query: Query<&mut LookTransform>,
    mut events: EventWriter<OutgoingReliableClientMessage<PawnClientMessage>>,
    mut prev_target: Local<Vec3>,
    state: Res<ActiveCamera>,
    pawn_id: Res<PawnId>,
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
            if *prev_target != look_transform.target {
                events.send(OutgoingReliableClientMessage {
                    message: PawnClientMessage::SyncLookTransform(look_transform.target),
                });
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
    mut messages: EventReader<IncomingReliableClientMessage<PawnClientMessage>>,
    handle_to_entity: Res<HandleToEntity>,
) {
    for msg in messages.iter() {
        match msg.message {
            PawnClientMessage::SyncLookTransform(target) => {
                match handle_to_entity.map.get(&msg.handle) {
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
                }
            }
        }
    }
}
