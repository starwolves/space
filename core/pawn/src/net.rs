use bevy::prelude::Vec3;
use serde::{Deserialize, Serialize};
use typename::TypeName;

#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]

pub enum UnreliableControllerClientMessage {
    UpdateLookTransform(Vec3),
}
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]

pub enum UnreliablePeerControllerClientMessage {
    UpdateLookTransform(Vec3, Vec3),
}

impl UnreliablePeerControllerClientMessage {
    pub fn from(message: UnreliableControllerClientMessage, position: Vec3) -> Self {
        match message {
            UnreliableControllerClientMessage::UpdateLookTransform(i) => {
                UnreliablePeerControllerClientMessage::UpdateLookTransform(i, position)
            }
        }
    }
}
