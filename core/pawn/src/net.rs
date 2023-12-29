use bevy::prelude::Vec3;
use serde::{Deserialize, Serialize};
use typename::TypeName;

#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]

pub enum UnreliableControllerClientMessage {
    UpdateLookTransform(Vec3, u8),
}
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]

pub enum UnreliablePeerControllerClientMessage {
    UpdateLookTransform(PeerUpdateLookTransform),
}
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PeerUpdateLookTransform {
    pub position: Vec3,
    pub target: Vec3,
    pub sub_tick: u8,
}

impl UnreliablePeerControllerClientMessage {
    pub fn from(message: UnreliableControllerClientMessage, position: Vec3) -> Self {
        match message {
            UnreliableControllerClientMessage::UpdateLookTransform(i, id) => {
                UnreliablePeerControllerClientMessage::UpdateLookTransform(
                    PeerUpdateLookTransform {
                        position,
                        target: i,
                        sub_tick: id,
                    },
                )
            }
        }
    }
}
