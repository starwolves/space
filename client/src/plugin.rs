use bevy::prelude::{App, Plugin};

/// The main plugin to add to execute the client.
pub struct ClientPlugin {
    pub version: String,
}

impl Default for ClientPlugin {
    fn default() -> Self {
        Self {
            version: "0.0.0".to_string(),
        }
    }
}

impl Plugin for ClientPlugin {
    fn build(&self, _app: &mut App) {}
}
