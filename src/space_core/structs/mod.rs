use serde::{Serialize, Deserialize};
use bevy::prelude::Color;
#[derive(Deserialize)]
pub struct WorldEnvironmentRaw;
#[derive(Serialize, Deserialize)]
pub struct WorldEnvironment;
