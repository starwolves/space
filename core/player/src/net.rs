use serde::{Deserialize, Serialize};
use typename::TypeName;

#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum PlayerServerMessage {
    InitGame,
    ServerTime,
    ConnectedPlayers(u16),
    ConfigTickRate(u8),
    PawnId(u64),
    Boarded,
    ConfigRepeatingSFX(String, Vec<String>),
    ConfigFinished,
    ConfigTalkSpaces(Vec<(String, String)>),
}
