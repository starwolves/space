use bevy::prelude::SystemLabel;
use serde::{Deserialize, Serialize};

/// Label for systems ordering.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ConsoleCommandsLabels {
    Finalize,
}

/// Variants for input console commands with values.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ConsoleCommandVariantValues {
    Int(i64),
    String(String),
    Float(f32),
    Bool(bool),
}
/// Variant types for input console commands with values.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ConsoleCommandVariant {
    Int,
    String,
    Float,
    Bool,
}

pub const CONSOLE_SUCCESS_COLOR: &str = "#3cff00";
pub const CONSOLE_ERROR_COLOR: &str = "#ff6600";
