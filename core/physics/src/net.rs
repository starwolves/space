use resources::physics::SmallCache;
use serde::{Deserialize, Serialize};
use typename::TypeName;

/// Gets serialized and sent over the net, this is the client message.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]
pub enum PhysicsUnreliableServerMessage {
    DesyncCheck(Vec<SmallCache>),
}
