use std::collections::HashMap;

use bevy::core::Timer;

pub struct AsanaBoardingAnnouncements {
    pub announcements : HashMap<String, Timer>
}
