use bevy_core::Timer;

// Logic works witha timer, better as resource.
#[derive(Default)]
pub struct AsanaBoardingAnnouncements {
    pub announcements: Vec<(String, Timer)>,
}
