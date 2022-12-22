use networking::server::GridMapLayer;
use serde::{Deserialize, Serialize};
use typename::TypeName;

use crate::networking::NetAction;

#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum ActionsServerMessage {
    TabData(Vec<NetAction>),
}

#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum ActionsClientMessage {
    TabDataEntity(u64),
    TabDataMap(GridMapLayer, i16, i16, i16),
    TabPressed(
        String,
        Option<u64>,
        Option<(GridMapLayer, i16, i16, i16)>,
        Option<u64>,
    ),
}
