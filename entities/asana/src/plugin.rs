use bevy::prelude::{App, Plugin, Update};
use resources::is_server::is_server;

use crate::tick_asana_boarding_announcements::tick_asana_boarding_announcements;

pub struct AsanaPlugin;

impl Plugin for AsanaPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_systems(Update, tick_asana_boarding_announcements);
        }
    }
}
