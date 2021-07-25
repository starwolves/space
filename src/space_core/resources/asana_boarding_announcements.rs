use bevy::{core::Timer, prelude::{FromWorld, World}};

pub struct AsanaBoardingAnnouncements {
    pub announcements : Vec<(String, Timer)>
}


impl FromWorld for AsanaBoardingAnnouncements {
    fn from_world(_world: &mut World) -> Self {
        AsanaBoardingAnnouncements {
            announcements : vec![],
        }
    }
}
