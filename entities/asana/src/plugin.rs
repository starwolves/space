use bevy::prelude::{App, Plugin};
use resources::is_server::is_server;

use super::tick_asana_boarding_announcements::tick_asana_boarding_announcements;

pub struct AsanaPlugin;

impl Plugin for AsanaPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_system(tick_asana_boarding_announcements);
        }
    }
}
