use bevy::prelude::{App, Plugin, Startup};

use crate::parse::init_token;

pub struct TokenPlugin;

impl Plugin for TokenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_token);
    }
}
