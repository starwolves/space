use bevy::ecs::system::ResMut;
use bevy::log::warn;
use bevy::{
    prelude::{EventReader, Local, Query, Res, SystemSet, Vec3},
    time::Time,
};

use bevy_renet::renet::RenetClient;
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

pub(crate) fn client_sync_look_transform(
    mut look_transform_query: Query<&mut LookTransform>,
    mut client: ResMut<RenetClient>,
    mut prev_target: Local<Vec3>,
    state: Res<ActiveCamera>,
    pawn_id: Res<PawnId>,
    physics_loop: Res<Time<Physics>>,
    stamp: Res<TickRateStamp>,
    typenames: Res<Typenames>,
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
            if *prev_target != look_transform.target && !physics_loop.is_paused() {
                /*info!(
                    "Sending target: {:?}:{}",
                    look_transform.target, stamp.large
                );*/

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
) {
    for msg in messages.read() {
        match msg.message {
            UnreliableControllerClientMessage::UpdateLookTransform(target) => {
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
