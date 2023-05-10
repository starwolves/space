use bevy::prelude::{App, Plugin};

use crate::parse::init_token;

pub struct TokenPlugin;

impl Plugin for TokenPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_token);
    }
}
