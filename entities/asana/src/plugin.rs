use bevy::prelude::{App, FixedUpdate, IntoSystemConfigs, Plugin};
use resources::{modes::is_server, sets::MainSet};

use crate::tick_asana_boarding_announcements::tick_asana_boarding_announcements;

pub struct AsanaPlugin;

impl Plugin for AsanaPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_systems(
                FixedUpdate,
                tick_asana_boarding_announcements.in_set(MainSet::Update),
            );
        }
    }
}
