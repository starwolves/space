use bevy::prelude::{App, Plugin, Update};

use crate::parse::init_token;

pub struct TokenPlugin;

impl Plugin for TokenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, init_token);
    }
}
