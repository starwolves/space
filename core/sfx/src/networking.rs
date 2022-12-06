use bevy::prelude::Vec3;
use serde::{Deserialize, Serialize};
use typename::TypeName;

/// Gets serialized and sent over the net, this is the server message.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum SfxServerMessage {
    PlaySound(String, f32, f32, Option<Vec3>),
}
