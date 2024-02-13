use bevy::ecs::system::{ResMut, Resource};
use bevy::log::warn;
use bevy::{
    prelude::{Local, Query, Res, SystemSet, Vec3},
    time::Time,
};

use bevy_renet::renet::RenetClient;
use bevy_xpbd_3d::prelude::{Physics, PhysicsTime};
use cameras::{controllers::fps::ActiveCamera, LookTransform};
use entity::spawn::PawnId;
use networking::messaging::{Typenames, UnreliableClientMessageBatch, UnreliableMessage};
use networking::plugin::RENET_UNRELIABLE_CHANNEL_ID;
use networking::stamp::TickRateStamp;
use typename::TypeName;

use crate::net::UnreliableControllerClientMessage;

/// Label for systems ordering.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum LookTransformSet {
    Sync,
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
pub(crate) fn mouse_input(
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
    if pawn_id.client.is_none() {
        return;
    }
    let lk;
    match look_transform_query.get(camera_entity) {
        Ok(look_transform) => {
            let difference = (*prev_target - look_transform.target).abs();
            if difference.length() > 0.0001 && !physics_loop.is_paused() {
                let large_target_tick;
                large_target_tick = stamp.tick + 1;

                let target_tick = TickRateStamp::new(large_target_tick).tick;
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
                        stamp: target_tick,
                        fixed: false,
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
