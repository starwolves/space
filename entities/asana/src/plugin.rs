use bevy::prelude::{App, Plugin};
use resources::is_server::is_server;

pub struct AsanaPlugin;

impl Plugin for AsanaPlugin {
    fn build(&self, _app: &mut App) {
        if is_server() {
            // app.add_system(tick_asana_boarding_announcements);
        }
    }
}
