use serde::{Deserialize, Serialize};
use typename::TypeName;

#[derive(TypeName, Serialize, Deserialize, Debug, Clone)]
pub enum BallClientMessage {
    Shoot,
}
