use serde::Deserialize;
use serde::Serialize;
use typename::TypeName;

use std::collections::HashMap;

use networking::server::EntityUpdateData;

use crate::entity_data::EntityWorldType;

/// Gets serialized and sent over the net, this is the client message.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum EntityClientMessage {
    ExamineEntity(u64),
}

/// Gets serialized and sent over the net, this is the server message.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum EntityServerMessage {
    EntityUpdate(
        u64,
        HashMap<String, HashMap<String, EntityUpdateData>>,
        bool,
        EntityWorldType,
    ),
    LoadEntity(u16, u64),
    UnloadEntity(u64),
}
