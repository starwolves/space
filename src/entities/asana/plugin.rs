use bevy::prelude::{App, Plugin};

use super::tick_asana_boarding_announcements::{
    tick_asana_boarding_announcements, AsanaBoardingAnnouncements,
};

pub struct AsanaPlugin;

impl Plugin for AsanaPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AsanaBoardingAnnouncements>()
            .add_system(tick_asana_boarding_announcements);
    }
}
