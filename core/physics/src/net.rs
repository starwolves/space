use serde::{Deserialize, Serialize};
use typename::TypeName;

use crate::cache::SmallCache;

/// Gets serialized and sent over the net, this is the client message.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]
pub enum PhysicsServerMessage {
    DesyncCheck(Vec<SmallCache>),
}
