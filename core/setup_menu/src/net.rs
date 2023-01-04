use serde::{Deserialize, Serialize};
use typename::TypeName;

#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum SetupUiServerMessage {
    SuggestedCharacterName(String),
    InitSetupUi,
}
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum SetupUiClientMessage {
    InputCharacterName(String),
    SetupUiLoaded,
    RequestBoarding,
}
