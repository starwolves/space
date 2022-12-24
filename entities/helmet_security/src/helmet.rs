use bevy::prelude::Component;

/// The component.
#[derive(Component)]
#[cfg(feature = "server")]
pub struct Helmet;
