use bevy::prelude::Component;
use bevy_renet::renet::ClientId;

#[derive(Clone)]

pub struct ShowcaseData {
    pub handle: ClientId,
}

/// Component for entities in the showcase.
#[derive(Component)]

pub struct Showcase {
    pub handle: ClientId,
}
