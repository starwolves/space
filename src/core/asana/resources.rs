use bevy_core::Timer;
use bevy_ecs::prelude::{FromWorld, World};

// Logic works witha timer, better as resource.
pub struct AsanaBoardingAnnouncements {
    pub announcements: Vec<(String, Timer)>,
}

impl FromWorld for AsanaBoardingAnnouncements {
    fn from_world(_world: &mut World) -> Self {
        AsanaBoardingAnnouncements {
            announcements: vec![],
        }
    }
}
