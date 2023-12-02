use bevy::prelude::Vec3;
use serde::{Deserialize, Serialize};
use typename::TypeName;

#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]

pub enum UnreliableControllerClientMessage {
    UpdateLookTransform(Vec3),
}
