use serde::{Deserialize, Serialize};
use typename::TypeName;

#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]

pub enum SetupUiServerMessage {
    SuggestedCharacterName(String),
    InitSetupUi,
}
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]

pub enum SetupUiClientMessage {
    InputCharacterName(String),
    SetupUiLoaded,
    RequestBoarding,
}
