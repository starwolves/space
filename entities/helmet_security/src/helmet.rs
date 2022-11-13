use bevy::prelude::Component;

/// The component.
#[derive(Component)]
#[cfg(feature = "server")]
pub struct Helmet;
#[cfg(feature = "server")]
pub const HELMET_SECURITY_ENTITY_NAME: &str = "helmetSecurity";
