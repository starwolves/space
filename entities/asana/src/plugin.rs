use bevy::prelude::{App, Plugin};

use super::tick_asana_boarding_announcements::tick_asana_boarding_announcements;

pub struct AsanaPlugin;

impl Plugin for AsanaPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(feature = "server") {
            app.add_system(tick_asana_boarding_announcements);
        }
    }
}
