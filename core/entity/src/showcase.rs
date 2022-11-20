use bevy::prelude::Component;

#[derive(Clone)]
#[cfg(feature = "server")]
pub struct ShowcaseData {
    pub handle: u64,
}

/// Component for entities in the showcase.
#[derive(Component)]
#[cfg(feature = "server")]
pub struct Showcase {
    pub handle: u64,
}
