use bevy::prelude::Entity;
use serde::{Deserialize, Serialize};
use typename::TypeName;

#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]

pub enum PlayerServerMessage {
    InitGame,
    ServerTime,
    ConnectedPlayers(u16),
    ConfigTickRate(u8),
    PawnId(Entity),
    Boarded,
    ConfigRepeatingSFX(String, Vec<String>),
    ConfigFinished,
}
