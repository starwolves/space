use bevy::prelude::Color;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetTextSection {
    pub text: String,
    pub font: u16,
    pub font_size: f32,
    pub color: Color,
}

pub const COMMUNICATION_FONT_SIZE: f32 = 14.;
pub const CONSOLE_ERROR_COLOR: Color = Color::srgb(1., 0.4, 0.);
pub const CONSOLE_SUCCESS_COLOR: Color = Color::srgb(0.23, 1., 0.);
