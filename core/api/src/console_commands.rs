use bevy::prelude::SystemLabel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ConsoleCommandsLabels {
    Finalize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ConsoleCommandVariantValues {
    Int(i64),
    String(String),
    Float(f32),
    Bool(bool),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ConsoleCommandVariant {
    Int,
    String,
    Float,
    Bool,
}

pub const CONSOLE_SUCCESS_COLOR: &str = "#3cff00";
pub const CONSOLE_ERROR_COLOR: &str = "#ff6600";
