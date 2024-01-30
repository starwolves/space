use bevy::prelude::{App, Plugin};
use resources::{modes::is_server_mode, ordering::Update};

use crate::tick_asana_boarding_announcements::tick_asana_boarding_announcements;

pub struct AsanaPlugin;

impl Plugin for AsanaPlugin {
    fn build(&self, app: &mut App) {
        if is_server_mode(app) {
            app.add_systems(Update, tick_asana_boarding_announcements);
        }
    }
}
