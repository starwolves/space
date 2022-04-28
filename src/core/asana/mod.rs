use bevy_app::{App, Plugin};

use self::{
    resources::AsanaBoardingAnnouncements,
    systems::tick_asana_boarding_announcements::tick_asana_boarding_announcements,
};

pub mod resources;
pub mod systems;

pub struct AsanaPlugin;

impl Plugin for AsanaPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AsanaBoardingAnnouncements>()
            .add_system(tick_asana_boarding_announcements);
    }
}
