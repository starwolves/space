use std::env;

use bevy::prelude::{App, Resource};

#[derive(Resource, Default, Debug)]
pub enum AppMode {
    #[default]
    Standard,
    Correction,
}

impl AppMode {
    pub fn is_correction_server(&self) -> bool {
        match self {
            AppMode::Standard => false,
            AppMode::Correction => true,
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
    is_server() || app.world.resource::<AppMode>().is_correction_server()
}

pub fn is_correction_mode(app: &mut App) -> bool {
    app.world.resource::<AppMode>().is_correction_server()
}
