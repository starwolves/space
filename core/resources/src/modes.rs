use std::env;

use bevy::prelude::{App, Resource};

#[derive(Resource, Default)]
pub enum Mode {
    #[default]
    Standard,
    Correction,
}

impl Mode {
    pub fn is_correction_server(&self) -> bool {
        match self {
            Mode::Standard => false,
            Mode::Correction => true,
        }
    }
}

pub fn is_server() -> bool {
    match env::args().nth(1) {
        Some(c) => {
            if c == "server" {
                true
            } else {
                false
            }
        }
        None => false,
    }
}

pub fn is_server_mode(app: &mut App) -> bool {
    is_server() || app.world.resource::<Mode>().is_correction_server()
}

pub fn is_correction_mode(app: &mut App) -> bool {
    app.world.resource::<Mode>().is_correction_server()
}
