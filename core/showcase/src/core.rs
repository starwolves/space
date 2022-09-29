use bevy::prelude::Component;

#[derive(Clone)]
pub struct ShowcaseData {
    pub handle: u64,
}

/// Component for entities in the showcase.
#[derive(Component)]
pub struct Showcase {
    pub handle: u64,
}
